use crate::enu::{Content, Method};
use crate::util::{parse_url_stream, parse_exprs, parse_args, parse_return_type};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;
use std::collections::HashMap;

pub struct ReqMeta {
    // Url is a token stream, so it can be retrieved by a variable.
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
    let url = match parse_url_stream(&attr) {
        Ok(url) => url,
        Err(err) => return err.into_compile_error().into(),
    };

    let config = parse_exprs(&attr);

    let stream = fn_impl(
        ReqMeta {
            url,
            method,
            config,
        },
        item,
    );
    match stream {
        Ok(stream) => stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Generate function code.
pub fn fn_impl(req_meta: ReqMeta, item_stream: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let url = req_meta.url;
    let method = req_meta.method.to_str();
    let config = req_meta.config;

    let mut config_keys = Vec::new();
    let mut config_values = Vec::new();
    for (k, v) in config {
        config_keys.push(k);
        config_values.push(v);
    }

    let mut item_fn = syn::parse::<syn::ItemFn>(item_stream)?;

    let sig = &mut item_fn.sig;
    let asyncness = &sig.asyncness;
    if asyncness.is_none() {
        return Err(syn::Error::new_spanned(sig.fn_token, "only support async fn"));
    }

    let vis = &item_fn.vis;
    let args = parse_args(sig)?;

    let header_names = find_content_names(&args, Content::HEADER);
    let header_vars = find_content_vars(&args, Content::HEADER);

    let path_names = find_content_names(&args, Content::PATH);
    let path_vars = find_content_vars(&args, Content::PATH);

    let query_names = find_content_names(&args, Content::QUERY);
    let query_vars = find_content_vars(&args, Content::QUERY);

    let form_names = find_content_names(&args, Content::FORM);
    let form_vars = find_content_vars(&args, Content::FORM);

    let body_vars = find_content_vars(&args, Content::BODY);

    if form_vars.len() > 0 && body_vars.len() > 0 {
        return Err(syn::Error::new_spanned(
            &sig.inputs,
            "request must have only one of body or form"));
    } else if body_vars.len() > 1 {
        return Err(syn::Error::new_spanned(
            &sig.inputs,
            "request must have only one body"));
    }

    let mut send_fn_call = quote! {send()};
    if !body_vars.is_empty() {
        let body_types: Vec<syn::Type> = args.iter()
            .filter(|a| a.content == Content::BODY)
            .map(|a| a.var_type.clone())
            .collect();
        send_fn_call = get_body_fn_call(
            body_types.get(0).unwrap(),
            body_vars.get(0).unwrap());
    } else if !form_vars.is_empty() {
        let form_types: Vec<syn::Type> = args.iter()
            .filter(|a| a.content == Content::FORM)
            .map(|a| a.var_type.clone())
            .collect();
        match get_form_fn_call(&form_names, &form_types, &form_vars) {
            Ok(fn_call) => {
                send_fn_call = fn_call;
            },
            Err(e) => {
                return Err(syn::Error::new_spanned(
                    &sig.inputs, e));
            }
        }
    }

    let return_args = parse_return_type(sig)?;
    if return_args.is_empty() {
        return Err(syn::Error::new_spanned(
            &sig.output,
            "function must have generic parameters"));
    }
    let return_type = return_args.get(0).unwrap();
    let return_fn = get_return_fn(return_type);

    let stream = quote! {
        #vis #sig {
            use std::collections::HashMap;
            use feignhttp::{HttpClient, HttpConfig, HttpRequest, HttpResponse};

            let mut config_map: HashMap<&str, String> = HashMap::new();
            #(
                config_map.insert(#config_keys, format!("{}", #config_values));
            )*

            let mut header_map: HashMap<&str, String> = HashMap::new();
            #(
                header_map.insert(#header_names, format!("{}", #header_vars));
            )*

            let mut path_map: HashMap<&str, String> = HashMap::new();
            #(
                path_map.insert(#path_names, format!("{}", #path_vars));
            )*

            let mut query_vec: Vec<(&str, String)> = Vec::new();
            #(
                query_vec.push((#query_names, format!("{}", #query_vars)));
            )*

            let url = feignhttp::util::replace_url(&format!("{}", #url), &path_map);

            let config = HttpConfig::from_map(config_map);

            let request = HttpClient::configure_request(&url, #method, config)
                .headers(header_map).query(&query_vec);

            let response = request.#send_fn_call.await?;
            let return_value: #return_type = response.#return_fn().await?;

            Ok(return_value)
        }
    };

    Ok(stream)
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

fn get_body_fn_call(body_type: &syn::Type, body_var: &syn::Ident) -> proc_macro2::TokenStream {
    let body_type_str = body_type.to_token_stream().to_string();
    return if body_type_str.contains("String") || body_type_str.contains("& str") {
        quote! {send_text(#body_var .to_string())}
    } else {
        quote! {send_json(& #body_var)}
    }
}

fn get_return_fn(return_type: &syn::Type) -> proc_macro2::TokenStream {
    let return_type_str = return_type.to_token_stream().to_string();
    let is_text = if return_type_str.contains("String") { true } else { false };
    return if is_text {
        quote! {text}
    } else {
        quote! {json}
    }
}

fn is_form_support_types(t: String) -> bool {
    return match t.as_str() {
        "bool" |
        "u8" | "u16" | "u32" | "u64" |
        "i8" | "i16" | "i32" | "i64" |
        "f32" | "f64" |
        "char" | "String" | "&str" => {
            true
        },
        _ => {
            false
        }
    }
}

fn get_form_fn_call(
    form_names: &Vec<String>,
    form_types: &Vec<syn::Type>,
    form_vars: &Vec<syn::Ident>,
)-> Result<proc_macro2::TokenStream, String> {
    if form_names.is_empty() {
        return Err("no form parameters".to_string());
    }
    return if form_names.len() == 1 {
        let form_name = form_names.get(0).unwrap();
        let form_type = form_types.get(0).unwrap();
        let form_var = form_vars.get(0).unwrap();
        match form_type {
            syn::Type::Path(t) => {
                let ty = t.to_token_stream().to_string();
                if is_form_support_types(ty) {
                    let mut token_str = "send_form(&vec![".to_string();
                    token_str.push_str("(");
                    token_str.push_str(&format!("\"{}\", format!(\"{{}}\", {})", form_name, form_var.to_string()));
                    token_str.push_str("),");
                    token_str.push_str("])");
                    Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap())
                } else {
                    Ok(quote! {send_form(& #form_var)})
                }
            },
            syn::Type::Reference(t) => {
                let ty = t.to_token_stream().to_string();
                if is_form_support_types(ty.replace(" ", "").replace("&", "")) {
                    return Err(format!("one form parameter only supports scalar types, &str, String or struct"));
                } else if ty.contains("& str") {
                    let mut token_str = "send_form(&vec![".to_string();
                    token_str.push_str("(");
                    token_str.push_str(&format!("\"{}\", format!(\"{{}}\", {})", form_name, form_var.to_string()));
                    token_str.push_str("),");
                    token_str.push_str("])");
                    return Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap())
                }
                Ok(quote! {send_form(& #form_var)})
            }
            _ => {
                Err(format!("non supports form parameter: `{}: {}`", form_name, form_type.to_token_stream()))
            }
        }
    } else {
        let mut token_str = "send_form(&vec![".to_string();
        for i in 0..form_names.len() {
            let form_name = form_names.get(i).unwrap();
            let form_type = form_types.get(i).unwrap();
            let form_var = form_vars.get(i).unwrap();
            let ty = form_type.to_token_stream().to_string().replace(" ", "");
            if !is_form_support_types(ty) {
                return Err(format!("two or more form parameters only supports scalar types, &str or String"));
            }
            token_str.push_str("(");
            token_str.push_str(&format!("\"{}\", format!(\"{{}}\", {})", form_name, form_var.to_string()));
            token_str.push_str("),");
        }
        token_str.push_str("])");
        Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap())
    }
}
