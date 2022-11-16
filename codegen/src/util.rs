use crate::enu::ArgType;
use crate::func::FnArg;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::{parse::Parse, Attribute, Field, Lit, PatType, Token, Type};

/// Parse url and return url token stream.
/// A URL can be an expression.
pub fn parse_url_stream(attr: &TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let attr_str = attr.to_string();
    if attr_str.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "no metadata assign",
        ));
    }

    let attrs = attr_str.split(",");
    let exprs: Vec<&str> = attrs.into_iter().map(|u| u).collect();
    // The default starting expression is the URL.
    if let Some(expr_str) = exprs.first() {
        let expr_str = expr_str.trim();
        let url_expr = syn::parse::<syn::Expr>(TokenStream::from_str(expr_str).unwrap())?;
        let url_stream = parse_url(&url_expr)?;

        // Skip the url.
        for i in 1..exprs.len() {
            let exp_str = exprs[i].trim();
            let url_expr = syn::parse::<syn::Expr>(TokenStream::from_str(exp_str).unwrap())?;

            match url_expr {
                // An assignment path: path = xxx.
                syn::Expr::Assign(assign) => {
                    let left = &assign.left;
                    if let syn::Expr::Path(ref k) = **left {
                        let key = k.path.segments.last().unwrap().ident.to_string();
                        if key == "path" {
                            let right = &assign.right;
                            // Url + path.
                            return Ok(quote!(#url_stream .to_string() + #right));
                        }
                    }
                }
                _ => {}
            }
        }
        return Ok(quote!(#url_stream .to_string()));
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "no metadata assign",
    ))
}

/// Parse and validate the url.
pub fn parse_url(url_expr: &syn::Expr) -> syn::Result<proc_macro2::TokenStream> {
    return match url_expr {
        // An assignment url: url = xxx.
        syn::Expr::Assign(assign) => {
            let left = &assign.left;
            if let syn::Expr::Path(ref k) = **left {
                let key = k.path.segments.last().unwrap().ident.to_string();
                if key != "url" {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "metadata url not specified",
                    ));
                }
            }
            let right = &assign.right;
            Ok(right.to_token_stream())
        }
        // A literal url: `"http://xxx"`.
        syn::Expr::Lit(lit) => Ok(lit.to_token_stream()),
        // A variable url: URL.
        syn::Expr::Path(path) => Ok(path.to_token_stream()),
        syn::Expr::Field(field) => Ok(field.to_token_stream()),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "metadata url is invalid",
        )),
    };
}

struct Metas(Vec<Meta>);
impl Parse for Metas {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = Vec::new();

        loop {
            if input.is_empty() {
                break;
            }

            out.push(input.parse()?);
        }
        Ok(Self(out))
    }
}
struct Meta(String, String);
impl Parse for Meta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let k: syn::Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let v: syn::Expr = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(Self(k.to_string(), expr_to_string(v)))
    }
}

fn expr_to_string(exp: syn::Expr) -> String {
    match exp {
        syn::Expr::Lit(l) => return lit_to_string(l.lit),
        _ => {}
    }
    exp.into_token_stream().to_string()
}

fn lit_to_string(lit: Lit) -> String {
    match lit {
        Lit::Str(s) => s.value(),
        Lit::Int(n) => n.to_string(),
        Lit::Float(f) => f.to_string(),
        Lit::Bool(b) => b.value().to_string(),
        Lit::Verbatim(v) => v.to_string(),
        _ => String::new(),
    }
}

pub fn parse_exprs_attribute(att: &Attribute) -> syn::Result<HashMap<String, String>> {
    let metas = att.parse_args::<Metas>()?;

    let map: HashMap<String, String> = metas.0.into_iter().map(|x| (x.0, x.1)).collect();
    Ok(map)
}

/// Parse assignment expression like n1=v1, n2=v2, etc.
pub fn parse_exprs(attr_str: &str) -> HashMap<String, String> {
    let mut expr_map = HashMap::new();
    if attr_str == "" {
        return expr_map;
    }

    let attrs = attr_str.split(",");
    let exprs: Vec<&str> = attrs.into_iter().map(|u| u).collect();
    for exp_str in exprs.into_iter() {
        let expr = match syn::parse_str::<Meta>(exp_str) {
            Ok(x) => x,
            Err(_) => return HashMap::new(),
        };
        expr_map.insert(expr.0, expr.1);
    }

    expr_map
}

pub fn parse_args_from_sig(sig: &mut syn::Signature) -> syn::Result<Vec<FnArg>> {
    let iter = sig
        .inputs
        .iter_mut()
        .flat_map(|x| match x {
            syn::FnArg::Typed(y) => Some(y),
            _ => None,
        })
        .map(|x| x.into());
    parse_args(iter, Some(ArgType::QUERY))
}

pub fn parse_args_from_struct(item_struct: &mut syn::DataStruct) -> syn::Result<Vec<FnArg>> {
    let iter = item_struct.fields.iter_mut().map::<PType, _>(|x| x.into());
    parse_args(iter, None)
}

struct PType<'a> {
    ty: &'a Type,
    name: String,
    attrs: &'a mut Vec<Attribute>,
}

impl<'a> From<&'a mut PatType> for PType<'a> {
    fn from(this: &'a mut PatType) -> Self {
        Self {
            ty: &this.ty,
            name: this.pat.to_token_stream().to_string(),
            attrs: &mut this.attrs,
        }
    }
}
impl<'a> From<&'a mut Field> for PType<'a> {
    fn from(this: &'a mut Field) -> Self {
        Self {
            ty: &this.ty,
            name: this.ident.to_token_stream().to_string(),
            attrs: &mut this.attrs,
        }
    }
}

fn extract_name(attr: &Attribute) -> Option<String> {
    let vec = get_metas(attr)?;
    let nested_meta = vec.first()?;
    match nested_meta {
        syn::NestedMeta::Lit(lit) => {
            if let syn::Lit::Str(lit) = lit {
                if !lit.value().is_empty() {
                    return lit.value().into();
                }
            }
        }
        _ => {
            if let Some(name_value) = get_meta_str_value(nested_meta, "name") {
                return name_value.into();
            }
        }
    }
    None
}

/// Parse function args.
fn parse_args<'a>(
    types: impl Iterator<Item = PType<'a>>,
    default_arg_type: Option<ArgType>,
) -> syn::Result<Vec<FnArg>> {
    let mut req_args: Vec<FnArg> = Vec::new();

    for pat_type in types {
        let name = pat_type.name;
        let ident = syn::Ident::new(&name.clone(), proc_macro2::Span::call_site());

        match &*pat_type.ty {
            syn::Type::Path(_) | syn::Type::Reference(_) | syn::Type::Array(_) => {}
            _ => {
                return Err(syn::Error::new_spanned(
                        quote!(),
                        "function args type must be like `std::slice::Iter`, `&std::slice::Iter` or `[T; n]`"));
            }
        }

        let mut found_one = false;
        for (ty, attr) in pat_type
            .attrs
            .iter()
            .flat_map(|x| x.path.get_ident().map(|u| (u, x)))
            .flat_map(|(x, att)| ArgType::from_str(&x.to_string()).map(|u| (u, att)))
        {
            found_one = true;
            let name = extract_name(attr).unwrap_or_else(|| name.clone());

            req_args.push(FnArg {
                arg_type: ty,
                name,
                var: ident.clone(),
                var_type: pat_type.ty.clone(),
            });
        }

        if let (Some(ref arg_type), false) = (&default_arg_type, found_one) {
            req_args.push(FnArg {
                arg_type: arg_type.clone(),
                name,
                var: ident,
                var_type: pat_type.ty.clone(),
            });
        }

        pat_type.attrs.retain(|x| {
            if let Some(i) = x.path.get_ident() {
                let i = i.to_string();
                !i.as_str().parse::<ArgType>().is_ok()
            } else {
                true
            }
        });
    }

    Ok(req_args)
}

/// Parse return type of function.
pub fn parse_return_type(sig: &syn::Signature) -> syn::Result<Vec<syn::Type>> {
    let output = &sig.output;
    let mut err_msg = "function must have a return value".to_string();
    if let syn::ReturnType::Type(.., t) = output {
        if let syn::Type::Path(ref t_path) = **t {
            if let Some(syn::PathSegment {
                ident,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args, ..
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
        err_msg = "return value must be Result".to_string();
    }
    Err(syn::Error::new_spanned(&sig, err_msg))
}

pub fn get_metas(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if let Ok(syn::Meta::List(mate_list)) = attr.parse_meta() {
        return Some(mate_list.nested.into_iter().collect());
    }
    None
}

pub fn get_meta_str_value(meta: &syn::NestedMeta, name: &str) -> Option<String> {
    match meta {
        // A literal, like the `"name"` in `#[param(p = "name")]`.
        syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
            let key = name_value.path.segments.last().unwrap().ident.to_string();
            if key == name {
                if let syn::Lit::Str(lit) = &name_value.lit {
                    return Some(lit.value());
                }
            }
        }
        _ => {}
    }
    None
}
