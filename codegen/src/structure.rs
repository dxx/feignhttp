use crate::enu::Method;
use crate::func::{client_fn_impl, fn_impl, FnMetadata};
use crate::util::{
    get_meta_str_value, get_metas, parse_exprs, parse_url_stream, remove_url_attr,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::DeriveInput;
use syn::{parse_macro_input, ItemImpl};

pub fn feign_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = match parse_url_stream(&attr) {
        Ok(url) => url,
        Err(err) => return err.into_compile_error().into(),
    };

    let meta_map = parse_exprs(&remove_url_attr(&attr.to_string()));
    let item_impl = parse_macro_input!(item as ItemImpl);
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

pub fn feign_client_impl(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);

    let gen = &derive.generics;
    let ident = &derive.ident;

    match derive.data {
        syn::Data::Struct(struc) => match client_fn_impl(struc) {
            Ok(x) => quote! {
                impl #gen ::feignhttp::FeignClient for #ident #gen {
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
        },
        None => None
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
        if let syn::ImplItem::Method(syn::ImplItemMethod { attrs, .. }) = item {
            if let Some(attr) = attrs.last() {
                let mut url = base_url.clone();
                let mut meta_map = base_meta.clone();
                let method_ident =
                    Method::from_str(&attr.path.segments.last().unwrap().ident.to_string());
                let method = match method_ident {
                    Ok(method) => method,
                    Err(err) => return Err(syn::Error::new_spanned(&attr.path, err)),
                };

                let fn_path = parse_fn_path(attr)?;
                if !fn_path.is_empty() {
                    url = quote!(#url + #fn_path);
                }

                // Override meta.
                let map = parse_fn_metas(attr);
                for (k, v) in map {
                    meta_map.insert(k, v);
                }

                let fn_stream = fn_impl(
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
    if let Some(vec) = get_metas(attr) {
        if let Some(nested_meta) = vec.first() {
            match nested_meta {
                // A literal, like the `"/xxx"` in `#[get("/xxx")]`.
                syn::NestedMeta::Lit(lit) => {
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

fn parse_fn_metas(attr: &syn::Attribute) -> HashMap<String, String> {
    let mut attr_map = HashMap::new();
    if let Some(metas) = get_metas(attr) {
        for meta in metas.into_iter() {
            match meta {
                // A literal, like the `xxx` in `#[get(p = xxx)]`.
                syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                    let key = name_value.path.segments.last().unwrap().ident.to_string();
                    match name_value.lit {
                        syn::Lit::Str(s) => {
                            attr_map.insert(key, s.value());
                        }
                        syn::Lit::Int(i) => {
                            attr_map.insert(key, i.to_string());
                        }
                        syn::Lit::Float(f) => {
                            attr_map.insert(key, f.to_string());
                        }
                        syn::Lit::Bool(b) => {
                            attr_map.insert(key, format!("{}", b.value()));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    attr_map
}
