use crate::enu::Method;
use crate::func::{fn_impl, ReqMeta};
use crate::util::{url_to_stream, get_metas, get_meta_str_value};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemImpl};

pub fn feign_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = url_to_stream(attr);
    if let Err(err) = url {
        return err.to_compile_error().into();
    }
    let url = url.unwrap();

    let item_impl = parse_macro_input!(item as ItemImpl);

    let fn_streams = fn_to_streams(url, item_impl.items);

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
) -> Vec<proc_macro2::TokenStream> {
    let base_url = url;
    let mut method_streams = Vec::new();
    for item in items.iter() {
        let mut url = base_url.clone();
        // Default get method
        let mut method = Method::GET;
        if let syn::ImplItem::Method(syn::ImplItemMethod { attrs, .. }) = item {
            if let Some(attr) = attrs.last() {
                let method_ident =
                    Method::from_str(&attr.path.segments.last().unwrap().ident.to_string());
                if let Err(err) = method_ident {
                    method_streams
                        .push(syn::Error::new_spanned(&attr.path, err).into_compile_error());
                    return method_streams;
                }
                method = method_ident.unwrap();

                let fn_path = parse_fn_path(attr);
                if !fn_path.is_empty() {
                    url = quote!(#url + #fn_path)
                }
            }
        }

        let func: proc_macro2::TokenStream = fn_impl(
            ReqMeta {
                url: url.clone(),
                method,
            },
            item.to_token_stream().into(),
        )
        .into();
        method_streams.push(func);
    }
    method_streams
}

fn parse_fn_path(attr: &syn::Attribute) -> proc_macro2::TokenStream {
    if let Ok(vec) = get_metas(attr) {
        if let Some(nested_meta) = vec.first() {
            return match get_meta_str_value(nested_meta, "path") {
                Some(val) => {
                    val.to_token_stream()
                },
                None => {
                    syn::Error::new_spanned(
                        nested_meta,
                        "metadata name can only be path",
                    ).to_compile_error()
                }
            }
        }
    }
    proc_macro2::TokenStream::new()
}
