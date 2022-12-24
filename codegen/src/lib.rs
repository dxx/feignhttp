mod enu;
mod func;
mod structure;
mod util;

use enu::Method;
use func::http_impl;
use proc_macro::TokenStream;
use structure::{feign_client_impl, feign_impl};

#[proc_macro_derive(Feign, attributes(url_path, query, header, param))]
pub fn feign_client(item: TokenStream) -> TokenStream {
    feign_client_impl(item)
}

#[proc_macro_attribute]
pub fn feign(attr: TokenStream, item: TokenStream) -> TokenStream {
    feign_impl(attr, item)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_impl(Method::GET, attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_impl(Method::POST, attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_impl(Method::PUT, attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_impl(Method::DELETE, attr, item)
}
