use crate::enu::Content;
use crate::func::ReqArg;
use quote::{quote, ToTokens};
use syn::{parse_macro_input};
use proc_macro::TokenStream;
use std::str::FromStr;

pub fn url_to_stream(attr: TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
    let attr_str = attr.to_string();
    if attr_str == "" {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), "no metadata assign"));
    }

    let url = attr_str.split(",");
    let url: Vec<&str> = url.into_iter().map(|u| u).collect();
    if let Some(exp) = url.first() {
        let exp_str = exp.trim();
        let url_expr = parse_macro_input::parse::<syn::Expr>(
            TokenStream::from_str(exp_str).unwrap()).unwrap();
        let url_stream = parse_url(&url_expr)?;

        for i in 1..url.len() {
            let exp_str = url[i].trim();
            let url_expr = parse_macro_input::parse::<syn::Expr>(
                TokenStream::from_str(exp_str).unwrap()).unwrap();

            match url_expr {
                // An assignment path: path = xxx
                syn::Expr::Assign(assign) => {
                    let left = &assign.left;
                    if let syn::Expr::Path(ref k) = **left {
                        let key = k.path.segments.last().unwrap().ident.to_string();
                        if key == "path" {
                            let right = &assign.right;
                            return Ok(quote!(#url_stream .to_string() + #right));
                        }
                    }
                },
                _ => {}
            }
        }
        return Ok(quote!(#url_stream .to_string()));
    }
    Err(syn::Error::new(proc_macro2::Span::call_site(), "no metadata assign"))
}

pub fn parse_url(url_expr: &syn::Expr) -> Result<proc_macro2::TokenStream, syn::Error> {
    return match url_expr {
        // An assignment url: url = xxx
        syn::Expr::Assign(assign) => {
            let left = &assign.left;
            if let syn::Expr::Path(ref k) = **left {
                let key = k.path.segments.last().unwrap().ident.to_string();
                if key != "url" {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "metadata url not assign",
                    ));
                }
            }
            let right = &assign.right;
            Ok(right.to_token_stream())
        }
        // A literal url: `"http://xxx"`
        syn::Expr::Lit(lit) => Ok(lit.to_token_stream()),
        // A variable url: URL
        syn::Expr::Path(path) => Ok(path.to_token_stream()),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "metadata url is invalid",
        )),
    };
}

pub fn parse_args(sig: &mut syn::Signature) -> Result<Vec<ReqArg>, syn::Error> {
    let input = &mut sig.inputs;
    let mut req_args: Vec<ReqArg> = Vec::new();
    for fn_arg in input.iter_mut() {
        if let syn::FnArg::Typed(pat_type) = fn_arg {
            let attrs = pat_type.attrs.clone();
            pat_type.attrs.clear();

            // Default param
            let mut content = Content::PARAM;
            let mut name = format!("{}", pat_type.pat.to_token_stream());
            let ident = syn::Ident::new(&name.clone(), proc_macro2::Span::call_site());
            match &*pat_type.ty {
                syn::Type::Path(_) | syn::Type::Reference(_) | syn::Type::Array(_) => {}
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pat_type,
                        "function args type must be like `std::slice::Iter`, `&std::slice::Iter` or `[T; n]`"));
                }
            }

            if let Some(attr) = attrs.last() {
                // Content: header, param, path, body
                let attr_ident =
                    Content::from_str(&attr.path.segments.last().unwrap().ident.to_string());
                if let Err(err) = attr_ident {
                    return Err(syn::Error::new_spanned(&attr.path, err));
                }
                content = attr_ident.unwrap();
                if let Ok(vec) = get_metas(attr) {
                    if let Some(nested_meta) = vec.first() {
                        if let Some(name_value) = get_meta_str_value(nested_meta, "name") {
                            name = name_value;
                        }
                    }
                }
            }

            req_args.push(ReqArg {
                content,
                name,
                var: ident,
                var_type: *pat_type.ty.clone(),
            });
        }
    }

    Ok(req_args)
}

pub fn parse_return_type(sig: &syn::Signature) -> Result<Vec<syn::Type>, syn::Error> {
    let output = &sig.output;
    let mut err_msg = "function must have a return value".to_string();
    if let syn::ReturnType::Type(.., t) = output {
        if let syn::Type::Path(ref t_path) = **t {
            if let Some(syn::PathSegment {
                ident,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }),
            }) = t_path.path.segments.last()
            {
                if ident.to_string() == "Result" {
                    let mut return_args = Vec::new();
                    for arg in args.iter() {
                        if let syn::GenericArgument::Type(t) = arg {
                            return_args.push(t.clone());
                        }
                    }
                    return Ok(return_args);
                }
            }
        }
        err_msg = "return value must be Result<?, ?>".to_string();
    }
    Err(syn::Error::new_spanned(&sig, err_msg))
}

pub fn get_metas(attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    if let Ok(syn::Meta::List(mate_list)) = attr.parse_meta() {
        return Ok(mate_list.nested.into_iter().collect());
    }
    Err(())
}

pub fn get_meta_str_value(meta: &syn::NestedMeta, name: &str) -> Option<String> {
    match meta {
        // A literal, like the `"name"` in `#[param(p = "name")]`
        syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
            let key = name_value.path.segments.last().unwrap().ident.to_string();
            if key == name {
                if let syn::Lit::Str(lit) = &name_value.lit {
                    return Some(lit.value());
                }
            }
        }
        // A literal, like the `"name"` in `#[param("name")]`
        syn::NestedMeta::Lit(lit) => {
            if let syn::Lit::Str(lit) = lit {
                return Some(lit.value());
            }
        }
        _ => {}
    }
    None
}