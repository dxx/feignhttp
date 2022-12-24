use crate::enu::{ArgType, Method};
use crate::util::{
    parse_args_from_sig, parse_args_from_struct, parse_exprs, parse_return_type, parse_url_stream,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::DataStruct;

const CONFIG_KEYS: [&str; 2] = ["connect_timeout", "timeout"];

pub struct FnMetadata {
    // Url is a token stream, so it can be retrieved by a variable.
    pub url: proc_macro2::TokenStream,
    pub method: Method,
    pub meta_map: HashMap<String, String>,
}

pub struct FnArg {
    pub arg_type: ArgType,
    pub name: String,
    pub var: syn::Ident,
    pub var_type: syn::Type,
}

pub fn http_impl(method: Method, attr: TokenStream, item: TokenStream) -> TokenStream {
    let url = match parse_url_stream(&attr) {
        Ok(url) => url,
        Err(err) => return err.into_compile_error().into(),
    };

    let meta_map = parse_exprs(&attr.to_string());

    let stream = fn_impl(
        FnMetadata {
            url,
            method,
            meta_map,
        },
        item,
        true,
    );
    match stream {
        Ok(stream) => stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

pub fn client_fn_impl(
    mut item_struct: DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let args = parse_args_from_struct(&mut item_struct)?;

    let header_names = find_type_names(&args, ArgType::HEADER, |_fn_arg| true);
    let header_vars = find_type_vars(&args, ArgType::HEADER, |_fn_arg| true);

    let path_names = find_type_names(&args, ArgType::PATH, |_fn_arg| true);
    let path_vars = find_type_vars(&args, ArgType::PATH, |_fn_arg| true);

    let query_names = find_type_names(&args, ArgType::QUERY, filter_query_array);
    let query_vars = find_type_vars(&args, ArgType::QUERY, filter_query_array);

    let (query_array_names, query_array_vars) = find_query_array(&args);

    let param_names = find_type_names(&args, ArgType::PARAM, |_fn_arg| true);
    let param_vars = find_type_vars(&args, ArgType::PARAM, |_fn_arg| true);

    let tokens = quote!(
        fn param_map(&self) -> ::std::collections::HashMap<&str, String> {
            let mut out = ::std::collections::HashMap::new();
            #(
                out.insert(#param_names, format!("{}", self.#param_vars));
            )*
            out
        }

        fn header_map(&self) -> ::std::collections::HashMap<std::borrow::Cow<str>, String> {
            let mut out = ::std::collections::HashMap::new();
            #(
                out.insert(std::borrow::Cow::Borrowed(#header_names), format!("{}", self.#header_vars));
            )*
            out
        }

        fn path_map(&self) -> ::std::collections::HashMap<&str, String> {
            let mut out = ::std::collections::HashMap::new();
            #(
                out.insert(#path_names, format!("{}", self.#path_vars));
            )*
            out
        }

        fn query_map(&self) -> Vec<(&str, String)> {
            let mut query_vec: Vec<(&str, String)> = Vec::new();
            #(
                query_vec.push((#query_names, format!("{}", self.#query_vars)));
            )*

            #(
                let query_array_name = #query_array_names;
                for query_array_var in self.#query_array_vars.iter() {
                    query_vec.push((query_array_name, format!("{}", query_array_var)));
                }
            )*
            query_vec
        }
    );

    Ok(tokens)
}

/// Generate function code.
pub fn fn_impl(
    metadata: FnMetadata,
    item_stream: TokenStream,
    empty_maps: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let url = metadata.url;
    let method = metadata.method.to_str();
    let meta_map = metadata.meta_map;

    let mut config_keys = Vec::new();
    let mut config_values = Vec::new();
    for (k, v) in meta_map.iter() {
        if !CONFIG_KEYS.contains(&k.as_str()) {
            continue;
        }
        config_keys.push(k);
        config_values.push(v);
    }

    let (header_keys, header_values) = match meta_map.get("headers") {
        Some(val) => parse_header_values(&val)?,
        None => (vec![], vec![]),
    };

    let mut item_fn = syn::parse::<syn::ItemFn>(item_stream)?;

    let sig = &mut item_fn.sig;
    let asyncness = &sig.asyncness;
    if asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            sig.fn_token,
            "only support async fn",
        ));
    }

    let vis = &item_fn.vis;
    let args = parse_args_from_sig(sig)?;

    let header_names = find_type_names(&args, ArgType::HEADER, |_fn_arg| true);
    let header_vars = find_type_vars(&args, ArgType::HEADER, |_fn_arg| true);

    let path_names = find_type_names(&args, ArgType::PATH, |_fn_arg| true);
    let path_vars = find_type_vars(&args, ArgType::PATH, |_fn_arg| true);

    let query_names = find_type_names(&args, ArgType::QUERY, filter_query_array);
    let query_vars = find_type_vars(&args, ArgType::QUERY, filter_query_array);

    let (query_array_names, query_array_vars) = find_query_array(&args);

    let form_names = find_type_names(&args, ArgType::FORM, |_fn_arg| true);
    let form_vars = find_type_vars(&args, ArgType::FORM, |_fn_arg| true);

    let param_names = find_type_names(&args, ArgType::PARAM, |_fn_arg| true);
    let param_vars = find_type_vars(&args, ArgType::PARAM, |_fn_arg| true);

    let body_vars = find_type_vars(&args, ArgType::BODY, |_fn_arg| true);

    // Valid form and body.
    if form_vars.len() > 0 && body_vars.len() > 0 {
        return Err(syn::Error::new_spanned(
            &sig.inputs,
            "request must have only one of body or form",
        ));
    } else if body_vars.len() > 1 {
        return Err(syn::Error::new_spanned(
            &sig.inputs,
            "request must have only one body",
        ));
    }

    // Valid param types.
    if param_names.len() > 0 {
        let param_types = find_var_types(&args, ArgType::PARAM);
        for i in 0..param_types.len() {
            let p_name = param_names.get(i).unwrap();
            let p_type = param_types.get(i).unwrap();
            let ty = p_type.to_token_stream().to_string().replace(" ", "");
            if !is_support_types(&ty) {
                return Err(syn::Error::new_spanned(
                    &sig.inputs,
                    format!(
                        "unsupported param parameter: `{}: {}`",
                        p_name,
                        p_type.to_token_stream()
                    ),
                ));
            }
        }
    }

    let mut send_fn_call = quote! {send()};
    if !body_vars.is_empty() {
        let body_types = find_var_types(&args, ArgType::BODY);
        send_fn_call = get_body_fn_call(body_types.get(0).unwrap(), body_vars.get(0).unwrap());
    } else if !form_vars.is_empty() {
        let form_types = find_var_types(&args, ArgType::FORM);
        match get_form_fn_call(&form_names, &form_types, &form_vars) {
            Ok(fn_call) => {
                send_fn_call = fn_call;
            }
            Err(e) => {
                return Err(syn::Error::new_spanned(&sig.inputs, e));
            }
        }
    }

    let return_args = parse_return_type(sig)?;
    if return_args.is_empty() {
        return Err(syn::Error::new_spanned(
            &sig.output,
            "function must have generic parameters",
        ));
    }
    let return_type = return_args.get(0).unwrap();
    let return_fn = get_return_fn(return_type);

    #[rustfmt::skip]
    let param_map = if empty_maps { quote! ( HashMap::new() ) } else { quote! ( self.param_map() ) };
    #[rustfmt::skip]
    let header_map = if empty_maps { quote! ( HashMap::new() ) } else { quote! ( self.header_map() ) };
    #[rustfmt::skip]
    let path_map = if empty_maps { quote! ( HashMap::new() ) } else { quote! ( self.path_map() ) };
    #[rustfmt::skip]
    let query_map = if empty_maps { quote! ( Vec::new() ) } else { quote! ( self.query_map() ) };

    let stream = quote! {
        #vis #sig {
            use feignhttp::FeignClient as _;
            use std::collections::HashMap;
            use feignhttp::{HttpClient, HttpConfig, HttpResponse, util};
            use std::borrow::Cow;

            let mut param_map: HashMap<&str, String> = #param_map;
            #(
                param_map.insert(#param_names, format!("{}", #param_vars));
            )*

            let mut config_map: HashMap<&str, String> = HashMap::new();
            #(
                config_map.insert(#config_keys, util::replace(#config_values, &param_map));
            )*

            let mut header_map: HashMap<Cow<str>, String> = #header_map;

            // Header in `#[get("", headers="")]` added before header in `#[header]` added.
            #(
                let key = util::replace(#header_keys, &param_map);
                let value = util::replace(#header_values, &param_map);
                header_map.insert(Cow::Owned(key), value);
            )*

            #(
                header_map.insert(Cow::Borrowed(#header_names), #header_vars.to_string());
            )*

            let mut path_map: HashMap<&str, String> = #path_map;
            #(
                path_map.insert(#path_names, #path_vars.to_string());
            )*

            let mut query_vec: Vec<(&str, String)> = #query_map;
            #(
                query_vec.push((#query_names, #query_vars.to_string()));
            )*

            #(
                let query_array_name = #query_array_names;
                for query_array_var in #query_array_vars {
                    query_vec.push((query_array_name, query_array_var.to_string()));
                }
            )*

            let url = util::replace(&format!("{}", #url), &path_map);

            let config = HttpConfig::from_map(config_map)?;

            let request = HttpClient::builder().url(&url).method(#method).config(config)
                .headers(header_map).query(query_vec).build()?;

            let response = request.#send_fn_call.await?;
            let return_value: #return_type = response.#return_fn().await?;

            Ok(return_value)
        }
    };

    Ok(stream)
}

fn find_type_names(
    args: &Vec<FnArg>,
    arg_type: ArgType,
    filter: impl Fn(&FnArg) -> bool,
) -> Vec<String> {
    args.iter()
        .filter(|a| a.arg_type == arg_type)
        .filter(|a| filter(a))
        .map(|a| a.name.clone())
        .collect()
}

fn find_type_vars(
    args: &Vec<FnArg>,
    arg_type: ArgType,
    filter: impl Fn(&FnArg) -> bool,
) -> Vec<syn::Ident> {
    args.iter()
        .filter(|a| a.arg_type == arg_type)
        .filter(|a| filter(a))
        .map(|a| a.var.clone())
        .collect()
}

fn find_var_types(args: &Vec<FnArg>, arg_type: ArgType) -> Vec<syn::Type> {
    args.iter()
        .filter(|a| a.arg_type == arg_type)
        .map(|a| a.var_type.clone())
        .collect()
}

fn filter_query_array(arg: &FnArg) -> bool {
    let t = arg.var_type.to_token_stream().to_string();
    if t.starts_with("& [")
        || t.starts_with("Vec")
        || t.starts_with("& Vec")
        || t.starts_with("std :: vec :: Vec")
    {
        return false;
    }
    true
}

fn find_query_array(args: &Vec<FnArg>) -> (Vec<String>, Vec<syn::Ident>) {
    let args = args
        .iter()
        .filter(|a| a.arg_type == ArgType::QUERY)
        .filter(|a| {
            let t = a.var_type.to_token_stream().to_string();
            if t.starts_with("& [")
                || t.starts_with("Vec")
                || t.starts_with("& Vec")
                || t.starts_with("std :: vec :: Vec")
            {
                return true;
            }
            false
        });
    let (mut names, mut vars) = (vec![], vec![]);
    for arg in args {
        names.push(arg.name.clone());
        vars.push(arg.var.clone());
    }

    (names, vars)
}

fn parse_header_values(s: &str) -> syn::Result<(Vec<String>, Vec<String>)> {
    let (mut key_vec, mut value_vec) = (vec![], vec![]);
    if s.len() <= 0 {
        return Ok((key_vec, value_vec));
    }
    let s_split = s.split(";");
    for header_str in s_split {
        let header_split = header_str.split(":");
        let header_vec: Vec<&str> = header_split.into_iter().collect();
        if header_vec.len() != 2 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("headers format is incorrect: {}", header_str),
            ));
        }
        let k = header_vec[0].trim().to_string();
        if k.len() == 0 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("headers format is incorrect: {}", header_str),
            ));
        }
        let v = header_vec[1].trim().to_string();
        if v.len() == 0 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("headers format is incorrect: {}", header_str),
            ));
        }
        key_vec.push(k);
        value_vec.push(v);
    }
    return Ok((key_vec, value_vec));
}

fn is_support_types(t: &str) -> bool {
    return match t {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64"
        | "char" | "String" | "&str" => true,
        _ => false,
    };
}

fn get_body_fn_call(body_type: &syn::Type, body_var: &syn::Ident) -> proc_macro2::TokenStream {
    let body_type_str = body_type.to_token_stream().to_string();
    if body_type_str.ends_with("Vec < u8 >") {
        return quote! {send_vec(#body_var)};
    };
    return if body_type_str.ends_with("String") || body_type_str.ends_with("& str") {
        quote! {send_text(#body_var .to_string())}
    } else {
        quote! {send_json(& #body_var)}
    };
}

fn get_return_fn(return_type: &syn::Type) -> proc_macro2::TokenStream {
    let return_type_str = return_type.to_token_stream().to_string();
    if return_type_str == "()" {
        return quote! {none};
    }
    if return_type_str.ends_with("Vec < u8 >") {
        return quote! {vec};
    }
    let is_text = if return_type_str.ends_with("String") {
        true
    } else {
        false
    };
    return if is_text {
        quote! {text}
    } else {
        quote! {json}
    };
}

fn get_form_fn_call(
    form_names: &Vec<String>,
    form_types: &Vec<syn::Type>,
    form_vars: &Vec<syn::Ident>,
) -> Result<proc_macro2::TokenStream, String> {
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
                if is_support_types(&ty) {
                    let mut token_str = "send_form(&vec![".to_string();
                    token_str.push_str("(");
                    token_str.push_str(&format!(
                        "\"{}\", format!(\"{{}}\", {})",
                        form_name,
                        form_var.to_string()
                    ));
                    token_str.push_str("),");
                    token_str.push_str("])");
                    Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap())
                } else {
                    Ok(quote! {send_form(& #form_var)})
                }
            }
            syn::Type::Reference(t) => {
                let ty = t.to_token_stream().to_string();
                if is_support_types(&ty.replace(" ", "").replace("&", "")) {
                    return Err(format!(
                        "one form parameter only supports scalar types, &str, String or struct"
                    ));
                } else if ty.contains("& str") {
                    let mut token_str = "send_form(&vec![".to_string();
                    token_str.push_str("(");
                    token_str.push_str(&format!(
                        "\"{}\", format!(\"{{}}\", {})",
                        form_name,
                        form_var.to_string()
                    ));
                    token_str.push_str("),");
                    token_str.push_str("])");
                    return Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap());
                }
                Ok(quote! {send_form(& #form_var)})
            }
            _ => Err(format!(
                "unsupported form parameter: `{}: {}`",
                form_name,
                form_type.to_token_stream()
            )),
        }
    } else {
        let mut token_str = "send_form(&vec![".to_string();
        for i in 0..form_names.len() {
            let form_name = form_names.get(i).unwrap();
            let form_type = form_types.get(i).unwrap();
            let form_var = form_vars.get(i).unwrap();
            let ty = form_type.to_token_stream().to_string().replace(" ", "");
            if !is_support_types(&ty) {
                return Err(format!(
                    "two or more form parameters only supports scalar types, &str or String"
                ));
            }
            token_str.push_str("(");
            token_str.push_str(&format!(
                "\"{}\", format!(\"{{}}\", {})",
                form_name,
                form_var.to_string()
            ));
            token_str.push_str("),");
        }
        token_str.push_str("])");
        Ok(proc_macro2::TokenStream::from_str(token_str.as_str()).unwrap())
    };
}
