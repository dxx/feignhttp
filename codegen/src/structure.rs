use crate::enu::Method;
use crate::func::{fn_impl, ReqMeta};
use crate::util::{parse_url_stream, parse_exprs, get_metas, get_meta_str_value};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemImpl};
use std::collections::HashMap;

pub fn feign_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = match parse_url_stream(&attr) {
        Ok(url) => url,
        Err(err) => return err.into_compile_error().into(),
    };

    let config = parse_exprs(&attr);

    let item_impl = parse_macro_input!(item as ItemImpl);

    let fn_streams = match fn_to_streams(url, item_impl.items, config) {
        Ok(streams) => streams,
        Err(err) => return err.into_compile_error().into(),
    };

    let ty = &item_impl.self_ty;

    let stream = quote! {
        impl #ty {
          #(#fn_streams)*
        }
    };

    stream.into()
}

fn fn_to_streams(
    url: proc_macro2::TokenStream,
    items: Vec<syn::ImplItem>,
    config: HashMap<String, String>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let base_url = url;
    let mut config = config;
    let mut method_streams = Vec::new();
    for item in items.iter() {
        let mut url = base_url.clone();
        // Default get method.
        let mut method = Method::GET;
        if let syn::ImplItem::Method(syn::ImplItemMethod { attrs, .. }) = item {
            if let Some(attr) = attrs.last() {
                let method_ident = Method::from_str(
                    &attr.path.segments.last().unwrap().ident.to_string());
                method = match method_ident {
                    Ok(method) => method,
                    Err(err) => return Err(syn::Error::new_spanned(&attr.path, err)),
                };

                let fn_path = parse_fn_path(attr)?;
                if !fn_path.is_empty() {
                    url = quote!(#url + #fn_path);
                }

                // Override configuration.
                let config_map = parse_fn_attrs(attr);
                for (k, v) in config_map {
                    config.insert(k, v);
                }
            }
        }

        let fn_stream = fn_impl(
            ReqMeta {
                config: config.clone(),
                url: url.clone(),
                method,
            },
            item.to_token_stream().into(),
        )?;
        method_streams.push(fn_stream);
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
                        Some(val) => {
                            Ok(val.to_token_stream())
                        },
                        None => {
                            Err(syn::Error::new_spanned(
                                nested_meta,
                                "metadata path not specified or must be the first",
                            ))
                        }
                    }
                }
            }
        }
    }
    Ok(proc_macro2::TokenStream::new())
}

fn parse_fn_attrs(attr: &syn::Attribute) -> HashMap<String, String> {
    let mut attr_map = HashMap::new();
    if let Some(metas) = get_metas(attr) {
        for meta in metas.into_iter() {
            match meta {
                // A literal, like the `xxx` in `#[get(p = xxx)]`.
                syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                    let key = name_value.path.segments.last().unwrap().ident.to_string();
                    attr_map.insert(key, name_value.lit.to_token_stream().to_string());
                }
                _ => {}
            }
        }
    }
    attr_map
}
