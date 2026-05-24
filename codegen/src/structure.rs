use crate::enu::Method;
use crate::func::{client_fn_impl, fn_impl, FnMetadata};
use crate::util::{
    get_meta_str_value, parse_attr_metas, parse_attr_meta_to_map, parse_exprs, parse_url_stream, remove_url_attr, NestedMeta,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::DeriveInput;
use syn::{parse_macro_input, ItemImpl};

pub fn feign_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = match parse_url_stream(&attr) {
        Ok(url) => url,
        Err(err) => return err.into_compile_error().into(),
    };

    let meta_map = parse_exprs(&remove_url_attr(&attr.to_string()));
    let item_parse = parse_macro_input!(item as syn::Item);

    match item_parse {
        syn::Item::Trait(item_trait) => {
            let trait_ident = &item_trait.ident;
            let builder_ident = format_ident!("{}Builder", trait_ident);

            let trait_fn_streams =
                match fn_to_streams_for_trait(url.clone(), &item_trait.items, meta_map.clone()) {
                    Ok(streams) => streams,
                    Err(err) => return err.into_compile_error().into(),
                };

            quote! {
                pub struct #builder_ident {
                    config: Option<::feignhttp::ClientConfig>,
                    context: Option<Box<dyn ::feignhttp::FeignContext>>,
                }

                impl ::feignhttp::FeignClientBuilder for #builder_ident {
                    type Target = #trait_ident;

                    fn new() -> Self {
                        Self {
                            config: None,
                            context: None,
                        }
                    }

                    fn config(mut self, config: ::feignhttp::ClientConfig) -> Self {
                        self.config = Some(config);
                        self
                    }

                    fn context<C>(mut self, context: C) -> Self
                    where
                        C: ::feignhttp::FeignContext + 'static,
                    {
                        self.context = Some(Box::new(context));
                        self
                    }

                    fn build(self) -> ::feignhttp::Result<Self::Target> {
                        let client = match self.config {
                            Some(config) => ::feignhttp::HttpClient::with_config(config)?,
                            None => ::feignhttp::HttpClient::new()?,
                        };
                        Ok(#trait_ident {
                            client,
                            context: self.context,
                        })
                    }
                }

                impl #builder_ident {
                    pub fn build_with_client(
                        client: ::feignhttp::ClientWrapper
                    ) -> ::feignhttp::Result<#trait_ident> {
                        Ok(#trait_ident {
                            client,
                            context: None,
                        })
                    }

                    pub fn build_with_context<C>(
                        client: ::feignhttp::ClientWrapper,
                        context: C
                    ) -> ::feignhttp::Result<#trait_ident>
                    where
                        C: ::feignhttp::FeignContext + 'static,
                    {
                        Ok(#trait_ident {
                            client,
                            context: Some(Box::new(context)),
                        })
                    }
                }

                pub struct #trait_ident {
                    client: ::feignhttp::ClientWrapper,
                    context: Option<Box<dyn ::feignhttp::FeignContext>>,
                }

                impl #trait_ident {
                    pub fn builder() -> #builder_ident {
                        #builder_ident::new()
                    }

                    #(#trait_fn_streams)*
                }

                impl ::feignhttp::FeignContext for #trait_ident {

                    fn param_map(&self) -> std::collections::HashMap<String, String> {
                        match &self.context {
                            Some(f) => (**f).param_map(),
                            None => std::collections::HashMap::new()
                        }
                    }

                    fn path_map(&self) -> std::collections::HashMap<String, String> {
                        match &self.context {
                            Some(f) => (**f).path_map(),
                            None => std::collections::HashMap::new()
                        }
                    }

                    fn header_map(&self) -> ::feignhttp::Result<std::collections::HashMap<String, String>> {
                        match &self.context {
                            Some(f) => (**f).header_map(),
                            None => Ok(std::collections::HashMap::new())
                        }
                    }

                    fn query_map(&self) -> ::feignhttp::Result<Vec<(String, String)>> {
                        match &self.context {
                            Some(f) => (**f).query_map(),
                            None => Ok(Vec::new())
                        }
                    }
                }
            }
            .into()
        }
        syn::Item::Impl(item_impl) => {
            eprint!("warning: useing `feign` on impl is deprecated, please use it on trait and it will be removed in future versions.\n");

            let impl_signature = impl_signature(&item_impl);

            let fn_streams = match fn_to_streams(url, item_impl.items, meta_map) {
                Ok(streams) => streams,
                Err(err) => return err.into_compile_error().into(),
            };

            let stream = quote! {
                #impl_signature {
                #(#fn_streams)*
                }
            };

            stream.into()
        }
        _ => syn::Error::new_spanned(item_parse, "Expected a trait or impl")
            .into_compile_error()
            .into(),
    }
}

pub fn feign_context_impl(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);

    let context_gen = &derive.generics;
    let ident = &derive.ident;

    match derive.data {
        syn::Data::Struct(struc) => match client_fn_impl(struc) {
            Ok(x) => quote! {
                impl #context_gen ::feignhttp::FeignContext for #ident #context_gen {
                    #x
                }
            }
            .into(),
            Err(e) => e.into_compile_error().into(),
        },
        _ => syn::Error::new_spanned(derive, "Expected a struct")
            .into_compile_error()
            .into(),
    }
}

fn impl_signature(item_impl: &ItemImpl) -> proc_macro2::TokenStream {
    let impl_token = &item_impl.impl_token;
    let generics = &item_impl.generics;
    let trait_token = match &item_impl.trait_ {
        Some(trait_) => {
            let t0 = &trait_.0;
            let t1 = &trait_.1;
            let t2 = &trait_.2;
            Some(quote! { #t0 #t1 #t2 })
        }
        None => None,
    };
    let self_ty = &item_impl.self_ty;

    quote! { #impl_token #generics #trait_token #self_ty }
}

fn fn_to_streams(
    url: proc_macro2::TokenStream,
    items: Vec<syn::ImplItem>,
    meta_map: HashMap<String, String>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let base_url = url;
    let base_meta = meta_map;
    let mut method_streams = Vec::new();
    for item in items.iter() {
        if let syn::ImplItem::Fn(syn::ImplItemFn { attrs, .. }) = item {
            if let Some(attr) = attrs.last() {
                let mut url = base_url.clone();
                let mut meta_map = base_meta.clone();
                let method_ident =
                    Method::from_str(&attr.path().segments.last().unwrap().ident.to_string());
                let method = match method_ident {
                    Ok(method) => method,
                    Err(err) => return Err(syn::Error::new_spanned(attr.path(), err)),
                };

                let fn_path = parse_fn_path(attr)?;
                if !fn_path.is_empty() {
                    url = quote!(#url + #fn_path);
                }

                // Override meta.
                let map = parse_attr_meta_to_map(attr);
                for (k, v) in map {
                    meta_map.insert(k, v);
                }

                #[allow(deprecated)]
                let fn_stream = crate::leagcy::leagcy_fn_impl(
                    FnMetadata {
                        url,
                        method,
                        meta_map,
                    },
                    item.to_token_stream().into(),
                    false,
                )?;
                method_streams.push(fn_stream);
                continue;
            }
        }

        method_streams.push(item.to_token_stream());
    }
    Ok(method_streams)
}

fn parse_fn_path(attr: &syn::Attribute) -> syn::Result<proc_macro2::TokenStream> {
    if let Some(vec) = parse_attr_metas(attr) {
        if let Some(nested_meta) = vec.first() {
            match nested_meta {
                // A literal, like the `"/xxx"` in `#[get("/xxx")]`.
                NestedMeta::Lit(lit) => {
                    if let syn::Lit::Str(lit) = lit {
                        return Ok(lit.value().to_token_stream());
                    }
                }
                _ => {
                    return match get_meta_str_value(nested_meta, "path") {
                        Some(val) => Ok(val.to_token_stream()),
                        None => Err(syn::Error::new_spanned(
                            nested_meta,
                            "metadata path not specified or must be the first",
                        )),
                    }
                }
            }
        }
    }
    Ok(proc_macro2::TokenStream::new())
}

fn fn_to_streams_for_trait(
    url: proc_macro2::TokenStream,
    items: &[syn::TraitItem],
    meta_map: HashMap<String, String>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let base_url = url;
    let base_meta = meta_map;
    let mut trait_fn_streams = Vec::new();

    for item in items.iter() {
        if let syn::TraitItem::Fn(trait_method) = item {
            if let Some(attr) = trait_method.attrs.last() {
                let mut url = base_url.clone();
                let mut meta_map = base_meta.clone();

                let method_ident =
                    Method::from_str(&attr.path().segments.last().unwrap().ident.to_string());
                let method = match method_ident {
                    Ok(method) => method,
                    Err(err) => return Err(syn::Error::new_spanned(attr.path(), err)),
                };

                let fn_path = parse_fn_path(attr)?;
                if !fn_path.is_empty() {
                    url = quote!(#url + #fn_path);
                }

                let map = parse_attr_meta_to_map(attr);
                for (k, v) in map {
                    meta_map.insert(k, v);
                }

                let item_fn = trait_method_to_item_fn(trait_method);
                let fn_stream = fn_impl(
                    FnMetadata {
                        url,
                        method,
                        meta_map,
                    },
                    item_fn.into_token_stream().into(),
                    false,
                )?;
                trait_fn_streams.push(fn_stream);
            }
        }
    }

    Ok(trait_fn_streams)
}

fn trait_method_to_item_fn(trait_method: &syn::TraitItemFn) -> syn::ItemFn {
    let sig = &trait_method.sig;

    syn::ItemFn {
        attrs: trait_method.attrs.clone(),
        vis: syn::Visibility::Inherited,
        sig: syn::Signature {
            constness: None,
            asyncness: sig.asyncness,
            unsafety: None,
            abi: None,
            fn_token: sig.fn_token,
            ident: sig.ident.clone(),
            generics: sig.generics.clone(),
            paren_token: sig.paren_token,
            inputs: sig.inputs.clone(),
            output: sig.output.clone(),
            variadic: None,
        },
        block: Box::new(syn::Block {
            stmts: vec![],
            brace_token: syn::token::Brace::default(),
        }),
    }
}
