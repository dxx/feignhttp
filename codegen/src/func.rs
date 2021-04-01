use crate::enu::{Content, Method};
use crate::util::{parse_url_stream, parse_exprs, parse_args, parse_return_type};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};
use std::collections::HashMap;

pub struct ReqMeta {
    pub url: proc_macro2::TokenStream,
    pub method: Method,
    pub config: HashMap<String, String>,
}

pub struct ReqArg {
    pub content: Content,
    pub name: String,
    pub var: syn::Ident,
    pub var_type: syn::Type,
}

pub fn http_impl(method: Method, attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = parse_url_stream(&attr);
    if let Err(err) = url {
        return err.to_compile_error().into();
    }

    let config_map = parse_exprs(&attr);

    fn_impl(
        ReqMeta {
            url: url.unwrap(),
            method,
            config: config_map,
        },
        item,
    )
}

pub fn fn_impl(req_meta: ReqMeta, item: TokenStream) -> TokenStream {
    let url = req_meta.url;
    let method = req_meta.method.to_str();
    let config = req_meta.config;

    let mut item_fn = parse_macro_input!(item as ItemFn);

    let sig = &mut item_fn.sig;
    let asyncness = &sig.asyncness;
    if asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only support async fn")
            .to_compile_error()
            .into();
    }

    let vis = &item_fn.vis;
    let args = parse_args(sig);
    if let Err(err) = args {
        return err.to_compile_error().into();
    }
    let args = args.unwrap();

    let header_names = find_content_names(&args, Content::HEADER);
    let header_vars = find_content_vars(&args, Content::HEADER);

    let query_names = find_content_names(&args, Content::PARAM);
    let query_vars = find_content_vars(&args, Content::PARAM);

    let path_names = find_content_names(&args, Content::PATH);
    let path_vars = find_content_vars(&args, Content::PATH);

    let body_vars = find_content_vars(&args, Content::BODY);
    if body_vars.len() > 1 {
        return syn::Error::new_spanned(
            &sig.inputs,
            "request must have only one body")
            .to_compile_error()
            .into();
    }
    let mut send_fn_call = quote! {send()};
    if !body_vars.is_empty() {
        let body_types: Vec<syn::Type> = args.iter()
            .filter(|a| a.content == Content::BODY)
            .map(|a| a.var_type.clone())
            .collect();
        let body_type_str = body_types.get(0).unwrap().to_token_stream().to_string();
        let body_var = body_vars.get(0).unwrap().clone();

        if body_type_str.contains("String") || body_type_str.contains("& str") {
            send_fn_call = quote! {send_text(#body_var .to_string())};
        } else {
            send_fn_call = quote! {send_json(& #body_var)};
        }
    }

    let mut config_keys = Vec::new();
    let mut config_values = Vec::new();
    for (k, v) in config {
        config_keys.push(k);
        config_values.push(v);
    }

    let return_type = parse_return_type(sig);
    if let Err(err) = return_type {
        return err.to_compile_error().into();
    }
    let return_args = return_type.unwrap();
    if return_args.is_empty() {
        return syn::Error::new_spanned(
            &sig.output,
            "function must have generic parameters")
            .to_compile_error()
            .into();
    }
    let return_type = return_args.get(0).unwrap();
    let return_type_str = return_type.to_token_stream().to_string();
    let is_text = if return_type_str.contains("String") { true } else { false };

    let ret_fn = if is_text {
        quote! {text}
    } else {
        quote! {json}
    };

    let stream = quote! {
        #vis #sig {
            use std::collections::HashMap;
            use feignhttp::{HttpClient, HttpConfig, HttpRequest, HttpResponse};

            let mut header_map: HashMap<&str, String> = HashMap::new();
            #(
                header_map.insert(#header_names, format!("{}", #header_vars));
            )*

            let mut query_vec: Vec<(&str, String)> = Vec::new();
            #(
                query_vec.push((#query_names, format!("{}", #query_vars)));
            )*

            let mut path_map: HashMap<&str, String> = HashMap::new();
            #(
                path_map.insert(#path_names, format!("{}", #path_vars));
            )*

            let mut config_map: HashMap<&str, String> = HashMap::new();
            #(
                config_map.insert(#config_keys, format!("{}", #config_values));
            )*

            let url = feignhttp::util::replace_url(&format!("{}", #url), &path_map);

            let config = HttpConfig::from_map(config_map);

            let request = HttpClient::configure_request(&url, #method, config)
                .headers(header_map).query(&query_vec);

            let response = request.#send_fn_call.await?;
            let return_value: #return_type = response.#ret_fn().await?;

            Ok(return_value)
        }
    };

    stream.into()
}

fn find_content_names(args: &Vec<ReqArg>, content: Content) -> Vec<String> {
    args.iter()
        .filter(|a| a.content == content)
        .map(|a| a.name.clone())
        .collect()
}

fn find_content_vars(args: &Vec<ReqArg>, content: Content) -> Vec<syn::Ident> {
    args.iter()
        .filter(|a| a.content == content)
        .map(|a| a.var.clone())
        .collect()
}
