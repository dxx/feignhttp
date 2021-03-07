mod enu;
mod func;
mod structure;
mod util;

use proc_macro::TokenStream;
use enu::Method;
use structure::feign_impl;
use func::http_impl;

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
