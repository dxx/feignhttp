mod enu;
mod func;
mod leagcy;
mod structure;
mod util;

use enu::Method;
use func::http_impl;
use leagcy::leagcy_feign_client_impl;
use proc_macro::TokenStream;
use structure::{feign_context_impl, feign_impl};

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

#[proc_macro_derive(Context, attributes(url_path, query, header, param))]
pub fn feign_context(item: TokenStream) -> TokenStream {
    feign_context_impl(item)
}

#[deprecated(
    since = "0.6.0",
    note = "`Feign` is deprecated, please use `Context` instead"
)]
#[proc_macro_derive(Feign, attributes(url_path, query, header, param))]
pub fn feign_client(item: TokenStream) -> TokenStream {
    leagcy_feign_client_impl(item)
}
