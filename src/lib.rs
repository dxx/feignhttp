//! # FeignHTTP
//!
//! FeignHTTP is a declarative HTTP client. Based on rust macros.
//!
//! Here are some features:
//!
//! * Easy to use
//! * Asynchronous request
//! * Supports form, plain text and JSON
//! * Configurable timeout settings
//! * Friendly error handling
//! * Selectable HTTP backends ([reqwest](https://docs.rs/reqwest) or [isahc](https://docs.rs/isahc))
//!
//! ## Table of contents
//!
//! * <a href="#usage">Usage</a>
//! * <a href="#making-a-post-request">Making a POST request</a>
//! * <a href="#paths">Paths</a>
//! * <a href="#url">URL</a>
//! * <a href="#query-parameters">Query Parameters</a>
//! * <a href="#headers">Headers</a>
//! * <a href="#form">Form</a>
//! * <a href="#json">JSON</a>
//! * <a href="#using-structure">Using Structure</a>
//! * <a href="#timeout-configuration">Timeout Configuration</a>
//! * <a href="#params">Params</a>
//! * <a href="#error-handling">Error Handling</a>
//! * <a href="#logs">Logs</a>
//! * <a href="#optional-features">Optional Features</a>
//!
//! ## Usage
//!
//! FeignHTTP mark macros on asynchronous functions, you need a runtime for support async/await. You can use [async-std](https://docs.rs/async-std) or [tokio](https://docs.rs/tokio).
//!
//! async-std:
//!
//! ```toml
//! [dependencies]
//! async-std = { version = "1", features = ["attributes", "tokio1"] }
//! ```
//!
//! The feature `tokio1` is need when use reqwest as the HTTP backend.
//!
//! tokio:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! Add `feignhttp` in your `Cargo.toml` and use default feature:
//!
//! ```toml
//! feignhttp = { version = "0.4" }
//! ```
//!
//! Then add the following code:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://api.github.com")]
//! async fn github() -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = github().await?;
//!     println!("result: {}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! The `get` attribute macro specifies get request, `feignhttp::Result<String>` specifies the return result.
//! It will send get request to `https://api.github.com` and receive a plain text body.
//!
//! Using non-default HTTP backend:
//!
//! ```toml
//! feignhttp = { version = "0.4", default-features = false, features = ["isahc-client"] }
//! ```
//!
//! The `default-features = false` option disable default reqwest.
//!
//! ## Making a POST request
//!
//! For a post request, you should use the `post` attribute macro to specify request method and use a `body` attribute to specify
//! a request body.
//!
//! ```rust, no_run
//! use feignhttp::post;
//!
//! #[post("https://httpbin.org/anything")]
//! async fn post_data(#[body] text: String) -> feignhttp::Result<String> {}
//! ```
//!
//! The `#[body]` mark a request body. Function parameter `text` is a String type, it will put in the request body as plain text.
//! String and &str will be put as plain text into the request body. Before send request, a header `content-type: text/plain` will be added automatically.
//!
//! ## Paths
//!
//! Using `path` to specify path value:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://api.github.com/repos/{owner}/{repo}")]
//! async fn repository(
//!     #[path("owner")] user: &str,
//!     #[path] repo: String,
//! ) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = repository("dxx", "feignhttp".to_string()).await?;
//!     println!("repository result: {}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! `dxx` will replace `{owner}` and `feignhttp` will replace `{repo}` , the url to be send will be
//! `https://api.github.com/repos/dxx/feignhttp`. You can specify a path name like `#[path("owner")]`.
//!
//! ## URL
//!
//! You can use constant to maintain all urls of request:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! const GITHUB_URL: &str = "https://api.github.com";
//!
//! #[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
//! async fn languages(
//!     #[path] owner: &str,
//!     #[path] repo: &str,
//! ) -> feignhttp::Result<String> {}
//! ```
//!
//! Url constant must be the first metadata in get attribute macro. You also can specify metadata key:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! const GITHUB_URL: &str = "https://api.github.com";
//!
//! #[get(url = GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
//! async fn languages(
//!     #[path] owner: &str,
//!     #[path] repo: &str,
//! ) -> feignhttp::Result<String> {}
//! ```
//!
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/url.rs) for more examples.
//!
//! ## Query Parameters
//!
//! Using `query` to specify query parameter:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
//! async fn contributors(
//!     #[path] owner: &str,
//!     #[path] repo: &str,
//!     #[query] page: u32,
//! ) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = contributors("dxx", "feignhttp", 1).await?;
//!     println!("contributors result: {}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! The `page` parameter will as query parameter in the url. An url which will be send is `https://api.github.com/repos/dxx/feignhttp?page=1`.
//!
//! **Note**: A function parameter without `query` attribute will as a query parameter by default.
//!
//! ## Headers
//!
//! Using `header` to specify request header:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://api.github.com/repos/dxx/feignhttp/commits")]
//! async fn commits(
//!     #[header] accept: &str,
//!     #[query] page: u32,
//!     #[query] per_page: u32,
//! ) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = commits("application/vnd.github.v3+json", 1, 5).await?;
//!     println!("commits result: {}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! A header `accept: application/vnd.github.v3+json` will be added.
//!
//! You also can use `headers` key to specify one or more headers in `get` attribute:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://httpbin.org/headers", headers = "key1: value1; key2: value2")]
//! async fn headers() -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = headers().await?;
//!     println!("headers result: {}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! The format of `headers` must be `header-key1: header-value1; header-key2: header-value2;...`.
//!
//! ## Form
//!
//! Using `form` to specify form parameter:
//!
//! ```rust, no_run
//! use feignhttp::post;
//!
//! #[post(url = "https://httpbin.org/anything")]
//! async fn post_user(
//!     #[form] id: i32,
//!     #[form] name: &str,
//! ) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = post_user(1, "jack").await?;
//!     println!("{}", r);
//!
//!     Ok(())
//! }
//! ```
//!
//! Before send request, a header `content-type: application/x-www-form-urlencoded` will be added automatically.
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/form.rs) for more examples.
//!
//! ## JSON
//!
//! [Serde](https://docs.rs/serde) is a framework for serializing and deserializing Rust data structures. When use json, you should add serde in `Cargo.toml`:
//!
//! ```toml
//! serde = { version = "1", features = ["derive"] }
//! ```
//!
//! You also need enable `json` feature:
//! ```toml
//! feignhttp = { version = "<version>", features = ["json"] }
//! ```
//!
//! Here is an example of getting json:
//!
//! ```rust, no_run
//! use feignhttp::get;
//! use serde::Deserialize;
//!
//! // Deserialize: Specifies deserialization
//! #[derive(Debug, Deserialize)]
//! struct IssueItem {
//!     pub id: u32,
//!     pub number: u32,
//!     pub title: String,
//!     pub url: String,
//!     pub repository_url: String,
//!     pub state: String,
//!     pub body: Option<String>,
//! }
//!
//! const GITHUB_URL: &str = "https://api.github.com";
//!
//! # #[cfg(feature = "json")]
//! #[get(url = GITHUB_URL, path = "/repos/{owner}/{repo}/issues")]
//! async fn issues(
//!     #[path] owner: &str,
//!     #[path] repo: &str,
//!     page: u32,
//!     per_page: u32,
//! ) -> feignhttp::Result<Vec<IssueItem>> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # #[cfg(feature = "json")]
//!     # {
//!     let r = issues("octocat", "hello-world", 1, 2).await?;
//!     println!("issues: {:#?}", r);
//!     # }
//!
//!     Ok(())
//! }
//! ```
//!
//! This issues function return `Vec<IssueItem>`, it is deserialized according to the content of the response.
//!
//! Send a json request:
//!
//! ```rust, no_run
//! use feignhttp::post;
//! use serde::Serialize;
//!
//! #[derive(Debug, Serialize)]
//! struct User {
//!     id: i32,
//!     name: String,
//! }
//!
//! # #[cfg(feature = "json")]
//! #[post(url = "https://httpbin.org/anything")]
//! async fn post_user(#[body] user: User) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # #[cfg(feature = "json")]
//!     # {
//!     let user = User {
//!         id: 1,
//!         name: "jack".to_string(),
//!     };
//!     let r = post_user(user).await?;
//!     println!("{}", r);
//!     # }
//!
//!     Ok(())
//! }
//! ```
//! Before send request, a header `content-type: application/json` will be added automatically.
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/json.rs) for a complete example.
//!
//! ## Using Structure
//!
//! Structure is a good way to manage requests. Define a structure and then define a large number of request methods：
//!
//! ```rust, no_run
//! use feignhttp::{feign, Feign};
//!
//! const GITHUB_URL: &str = "https://api.github.com";
//!
//! #[derive(Feign)]
//! struct Github;
//!
//! #[feign(url = GITHUB_URL)]
//! impl Github {
//!     #[get]
//!     async fn home(&self) -> feignhttp::Result<String> {}
//!
//!     #[get("/repos/{owner}/{repo}")]
//!     async fn repository(
//!         &self,
//!         #[path("owner")] user: &str,
//!         #[path] repo: &str,
//!     ) -> feignhttp::Result<String> {}
//!
//!     // ...
//!     
//!     // Structure method still send request
//!     #[get(path = "/repos/{owner}/{repo}/languages")]
//!     async fn languages(
//!         &self,
//!         #[path] owner: &str,
//!         #[path] repo: &str,
//!     ) -> feignhttp::Result<String> {}
//! }
//! ```
//!
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/struct.rs) for a complete example.
//!
//! ## Timeout Configuration
//!
//! If you need to configure the timeout, use `connect_timeout` and `timeout` to specify connect timeout and read timeout.
//!
//! Connect timeout:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get(url = "http://site_dne.com", connect_timeout = 3000)]
//! async fn connect_timeout() -> feignhttp::Result<String> {}
//! ```
//!
//! Read timeout:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
//! async fn timeout() -> feignhttp::Result<String> {}
//! ```
//!
//! ## Params
//!
//! Sometimes you need dynamic values, like config or others. `param` is designed to support such ability. You can use
//! `param` to specify a value that used as a dynamic replacement.
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get(url = "https://httpbin.org/delay/5", timeout = "{time}")]
//! async fn timeout(#[param] time: u16) -> feignhttp::Result<String> {}
//! ```
//!
//! When call `timeout` function, the time's value will replace the `{time}`.
//!
//! Another example is replace headers:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("https://httpbin.org/headers", headers = "token: {token}")]
//! async fn headers(#[param] token: &str) -> feignhttp::Result<String> {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // A reqeut with a header `token: ZmVpZ25odHRw`.
//!     let res = headers("ZmVpZ25odHRw").await?;
//!     println!("headers: {}", res);
//!
//!     Ok(())
//! }
//! ```
//!
//! **Note**: `param` can't replace placeholder in url or path.
//!
//! ## Error Handling
//!
//! FeignHTTP use [`feignhttp::Result`](https://docs.rs/feignhttp/latest/feignhttp/type.Result.html) to receive return result. The error is
//! [`Error`](https://docs.rs/feignhttp/latest/feignhttp/struct.Error.html) struct which has some error kinds and some useful methods.
//! [`ErrorKind`](https://docs.rs/feignhttp/latest/feignhttp/enum.ErrorKind.html) is used to indicate an error type.
//!
//! Url is incorrect:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get("httpbin.org/anything")]
//! async fn url_error() -> feignhttp::Result<()> {}
//!
//! #[tokio::main]
//! async fn main() {
//!     match url_error().await {
//!         Err(err) => {
//!             // Build client error.
//!             if err.is_build_error() {
//!                 println!("url_error: {}", err);
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//!
//! Parse config error:
//!
//! ```rust, no_run
//! use feignhttp::get;
//!
//! #[get(url = "https://httpbin.org/delay/3", timeout = "abc")]
//! async fn config_error() -> feignhttp::Result<()> {}
//!
//! #[tokio::main]
//! async fn main() {
//!     match config_error().await {
//!         Err(err) => {
//!             // Parse config error.
//!             if err.is_config_error() {
//!                 println!("config_error: {}", err);
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//! When parsing the configuration, an error is thrown if the value is incorrect. `timeout` is an integer type, when parse `abc` to integer will throw an error.
//!
//! HTTP status is an importmant info about response. The status code can tell whether the client or server is abnormal.
//! The following is an example of handling through HTTP status:
//!
//! ```rust, no_run
//! use feignhttp::{get, ErrorKind};
//!
//! #[get(url = "https://httpbin.org/123")]
//! async fn status_error() -> feignhttp::Result<()> {}
//!
//! #[tokio::main]
//! async fn main() {
//!     match status_error().await {
//!         Err(err) => {
//!             // Status error.
//!             if err.is_status_error() {
//!                 println!("status_error: {}", err);
//!             }
//!             if let ErrorKind::Status(status) = err.error_kind() {
//!             
//!                 println!("status error code: {}", status.as_u16());
//!
//!                 if status.is_client_error() {
//!                     // Handle error.
//!                 }
//!                 if status.is_server_error() {
//!                     // Handle error.
//!                 }
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//! The status is [StatusCode](https://docs.rs/http/latest/http/status/struct.StatusCode.html) struct that supply by [http](https://crates.io/crates/http) crate.
//! For more examples, see [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/error.rs).
//!
//! ## Logs
//!
//! FeignHTTP logs some useful information about requests and responses with the [log](https://crates.io/crates/log) crate.
//! To enable the log information, specify `log` feature in `Cargo.toml`, then set the log level to debug.
//!
//! ```toml
//! features = ["log"]
//! ```
//!
//! ## Optional Features
//!
//! The following features are available. The default features are `reqwest-client`
//! * **reqwest-client** *(default)*: Use `reqwest` as the HTTP backend
//! * **isahc-client**: Use `isahc` as the HTTP backend
//! * **json**: Enable json serialization and deserialization
//! * **log**: Enable request and response logs

mod error;
mod http;
mod macros;

#[cfg(feature = "reqwest-client")]
mod reqwest;

#[cfg(feature = "reqwest-client")]
pub use crate::reqwest::*;

#[cfg(feature = "isahc-client")]
mod isahc;
#[cfg(feature = "isahc-client")]
pub use crate::isahc::*;

#[doc(hidden)]
pub mod util;

use std::collections::HashMap;
pub use feignhttp_codegen::*;

pub use crate::error::{Error, ErrorKind, Result};
pub use crate::http::*;

pub trait FeignClient {
    fn param_map(&self) -> HashMap<&str, String>;
    fn header_map(&self) -> HashMap<std::borrow::Cow<str>, String>;
    fn path_map(&self) -> HashMap<&str, String>;
    fn query_map(&self) -> Vec<(&str, String)>;
}
