use crate::enu::{ArgType};
use crate::func::{FnArg, FnMetadata, client_fn_impl};
use crate::util::{
    parse_args_from_sig, parse_return_type,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, parse_macro_input};
use std::str::FromStr;

const CONFIG_KEYS: [&str; 1] = ["timeout"];

pub fn leagcy_feign_client_impl(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);

    let gen = &derive.generics;
    let ident = &derive.ident;

    match derive.data {
        syn::Data::Struct(struc) => match client_fn_impl(struc) {
            Ok(x) => quote! {
                impl #gen ::feignhttp::FeignContext for #ident #gen {
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

/// Generate function code.
#[deprecated(since = "0.6.0", note = "use `feign` on impl is deprecated, please use it on trait")]
pub fn leagcy_fn_impl(
    metadata: FnMetadata,
    item_stream: TokenStream,
    empty_maps: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let url = metadata.url;
    let method = metadata.method.to_str();
    let meta_map = metadata.meta_map;

    let mut item_fn = syn::parse::<syn::ItemFn>(item_stream)?;

    let sig = &mut item_fn.sig;

    let mut config_keys = Vec::new();
    let mut config_values = Vec::new();
    for (k, v) in meta_map.iter() {
        let key = k.as_str();
        if key == "connect_timeout" || key == "read_timeout" {
            return Err(syn::Error::new_spanned(
                sig.fn_token,
                format!("`{}` is not support on method or impl, please use trait instead", key),
            ));
        }
        if !CONFIG_KEYS.contains(&key) {
            continue;
        }
        config_keys.push(k);
        config_values.push(v);
    }

    let (header_keys, header_values) = match meta_map.get("headers") {
        Some(val) => parse_header_values(&val)?,
        None => (vec![], vec![]),
    };


    let asyncness = &sig.asyncness;
    if asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            sig.fn_token,
            "only support async fn",
        ));
    }

    let vis = &item_fn.vis;
    let args = parse_args_from_sig(sig)?;

    let (param_names, param_vars) = find_type_name_vars(&args, ArgType::PARAM, |_fn_arg| true);

    let (path_names, path_vars) = find_type_name_vars(&args, ArgType::PATH, |_fn_arg| true);

    let (header_names, header_vars) = find_type_name_vars(&args, ArgType::HEADER, filter_struct);

    let (_header_struct_names, header_struct_vars) =
        find_type_name_vars(&args, ArgType::HEADER, |fn_arg| !filter_struct(fn_arg));

    let (query_names, query_vars) =
        find_type_name_vars(&args, ArgType::QUERY, |fn_arg| filter_query_array(fn_arg) && filter_struct(fn_arg));

    let (query_array_names, query_array_vars) =
        find_type_name_vars(&args, ArgType::QUERY, |fn_arg| !filter_query_array(fn_arg));

    let (_query_struct_names, query_struct_vars) =
        find_type_name_vars(&args, ArgType::QUERY, |fn_arg| !filter_struct(fn_arg));

    let (form_names, form_vars) = find_type_name_vars(&args, ArgType::FORM, |_fn_arg| true);

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
        let param_types = find_arg_types(&args, ArgType::PARAM);
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
        let body_types = find_arg_types(&args, ArgType::BODY);
        send_fn_call = get_body_fn_call(body_types.get(0).unwrap(), body_vars.get(0).unwrap());
    } else if !form_vars.is_empty() {
        let form_types = find_arg_types(&args, ArgType::FORM);
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
    let path_map = if empty_maps { quote! ( HashMap::new() ) } else { quote! ( self.path_map() ) };
    #[rustfmt::skip]
    let header_map = if empty_maps { quote! ( HashMap::new() ) } else { quote! ( self.header_map()? ) };
    #[rustfmt::skip]
    let query_map = if empty_maps { quote! ( Vec::new() ) } else { quote! ( self.query_map()? ) };

    let stream = quote! {
        #vis #sig {
            use feignhttp::FeignContext as _;
            use std::collections::HashMap;
            use feignhttp::{HttpClient, RequestConfig, RequestBuilder, HttpResponse, ser, util};

            let mut param_map: HashMap<&str, String> = #param_map;
            #(
                param_map.insert(#param_names, format!("{}", #param_vars));
            )*

            let mut config_map: HashMap<&str, String> = HashMap::new();
            #(
                config_map.insert(#config_keys, util::replace(#config_values, &param_map));
            )*

            let mut header_map: HashMap<&str, String> = #header_map;

            // Header in `#[get("", headers="")]` added before header in `#[header]` added.
            #(
                let key = util::replace(#header_keys, &param_map);
                let value = util::replace(#header_values, &param_map);
                header_map.insert(key.as_str(), value);
            )*

            #(
                header_map.insert(#header_names, #header_vars.to_string());
            )*

            #(
                let map = ser::to_map(& #header_struct_vars)?;
                for (key, value) in map {
                    header_map.insert(key.as_str(), value);
                }
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

            #(
                let map = ser::to_map(& #query_struct_vars)?;
                for (key, value) in map.iter() {
                    query_vec.push((key.as_str(), value.to_string()));
                }
            )*

            let url = util::replace(&format!("{}", #url), &path_map);

            let config = RequestConfig::from_map(config_map)?;
            
            let client = HttpClient::shared();
            let request = RequestBuilder::new(client.clone())
                .url(&url)
                .method(#method)
                .config(config)
                .headers(header_map)
                .query(query_vec)
                .build()?;

            let response = request.#send_fn_call.await?;
            let return_value: #return_type = response.#return_fn().await?;

            Ok(return_value)
        }
    };

    Ok(stream)
}

fn find_type_name_vars(
    args: &Vec<FnArg>,
    arg_type: ArgType,
    filter: impl Fn(&FnArg) -> bool,
) -> (Vec<String>, Vec<syn::Ident>) {
    let args = args
        .iter()
        .filter(|arg| arg.arg_type == arg_type)
        .filter(|arg| filter(arg));
    let (mut names, mut vars) = (vec![], vec![]);
    for arg in args {
        names.push(arg.name.clone());
        vars.push(arg.var.clone());
    }
    (names, vars)
}

fn find_type_vars(
    args: &Vec<FnArg>,
    arg_type: ArgType,
    filter: impl Fn(&FnArg) -> bool,
) -> Vec<syn::Ident> {
    args.iter()
        .filter(|arg| arg.arg_type == arg_type)
        .filter(|arg| filter(arg))
        .map(|arg: &FnArg| arg.var.clone())
        .collect()
}

fn find_arg_types(args: &Vec<FnArg>, arg_type: ArgType) -> Vec<syn::Type> {
    args.iter()
        .filter(|arg| arg.arg_type == arg_type)
        .map(|arg| arg.var_type.clone())
        .collect()
}

fn filter_query_array(arg: &FnArg) -> bool {
    let ty = arg.var_type.to_token_stream().to_string();
    !is_sequences(&ty.replace(" ", ""))
}

fn filter_struct(arg: &FnArg) -> bool {
    let var_type = &arg.var_type;
    match var_type {
        syn::Type::Path(t) => {
            let ty = t.to_token_stream().to_string();
            !is_support_struct(&ty.replace(" ", ""))
        }
        syn::Type::Reference(t) => {
            let ty = t.to_token_stream().to_string();
            !is_support_struct(&ty.replace(" ", ""))
        }
        _ => true,
    }
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

fn is_support_struct(t: &str) -> bool {
    if is_support_types(t) || is_sequences(t) {
        false
    } else {
        true
    }
}

fn is_sequences(t: &str) -> bool {
    if t.starts_with("&[")
        || t.starts_with("Vec")
        || t.starts_with("&Vec")
        || t.starts_with("std::vec::Vec")
    {
        return true;
    }
    false
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
