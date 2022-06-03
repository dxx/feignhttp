//! # FeignHttp
//!
//! FeignHttp is a declarative HTTP client. Based on rust macros.
//! 
//! Here are some features:
//! 
//! * Easy to use
//! * Asynchronous request
//! * Configurable timeout settings
//! * Supports form, plain text and JSON
//! * Selectable HTTP backends ([reqwest](https://docs.rs/reqwest) or [isahc](https://docs.rs/isahc))
//! 
//! ## Table of contents
//! 
//! * <a href="#usage">Usage</a>
//! * <a href="#making-a-post-request">Making a POST request</a>
//! * <a href="#paths">Paths</a>
//! * <a href="#query-parameters">Query Parameters</a>
//! * <a href="#headers">Headers</a>
//! * <a href="#form">Form</a>
//! * <a href="#url-constant">URL Constant</a>
//! * <a href="#json">JSON</a>
//! * <a href="#using-structure">Using Structure</a>
//! * <a href="#timeout-configuration">Timeout Configuration</a>
//! * <a href="#logs">Logs</a>
//! * <a href="#optional-features">Optional Features</a>
//! 
//! ## Usage
//! 
//! FeignHttp mark macros on asynchronous functions, you need a runtime for support async/await. You can use [async-std](https://docs.rs/async-std) or [tokio](https://docs.rs/tokio).
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
//! Add feignhttp in your `Cargo.toml` and use default feature:
//!
//! ```toml
//! feignhttp = { version = "0.3" }
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
//! feignhttp = { version = "0.3", default-features = false, features = ["isahc-client"] }
//! ```
//!
//! The `default-features = false` option disable default reqwest.
//! 
//! ### Making a POST request
//! 
//! For a post request, you should use the `post` attribute macro to specify request method and use a `body` attribute to specify 
//! a request body.
//! 
//! ```rust, no_run
//! use feignhttp::post;
//! 
//! #[post(url = "https://httpbin.org/anything")]
//! async fn post_data(#[body] text: String) -> feignhttp::Result<String> {}
//! ```
//! 
//! The `#[body]` mark a request body. Function parameter `text` is a String type, it will put in the request body as plain text. 
//! String and &str will be put as plain text into the request body.
//! 
//! ### Paths
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
//! ### Query Parameters
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
//!     let r = contributors (
//!         "dxx",
//!         "feignhttp",
//!         1,
//!     ).await?;
//!     println!("contributors result: {}", r);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! The `page` parameter will as query parameter in the url. An url which will be send is `https://api.github.com/repos/dxx/feignhttp?page=1`.
//! 
//! Note: A function parameter without `query` attribute will as a query parameter by default.
//! 
//! ### Headers
//! 
//! Using `header` to specify request header:
//! 
//! ```rust, no_run
//! use feignhttp::get;
//! 
//! #[get("https://api.github.com/repos/{owner}/{repo}/commits")]
//! async fn commits(
//!     #[header] accept: &str,
//!     #[path] owner: &str,
//!     #[path] repo: &str,
//!     #[query] page: u32,
//!     #[query] per_page: u32,
//! ) -> feignhttp::Result<String> {}
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let r = commits(
//!         "application/vnd.github.v3+json",
//!         "dxx",
//!         "feignhttp",
//!         1,
//!         5,
//!     )
//!     .await?;
//!     println!("commits result: {}", r);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! A header `accept:application/vnd.github.v3+json ` will be send.
//!
//! ### Form
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
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/form.rs) for more examples.
//!
//! ### URL Constant
//! 
//! We can use constant to maintain all urls of request:
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
//! ### JSON
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
//!     # #[cfg(feature = "json")] {
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
//!     # #[cfg(feature = "json")] {
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
//! 
//! See [here](https://github.com/dxx/feignhttp/blob/HEAD/examples/json.rs) for a complete example.
//! 
//! ### Using Structure
//! 
//! Structure is a good way to manage requests. Define a structure and then define a large number of request methods：
//! 
//! ```rust, no_run
//! use feignhttp::feign;
//! 
//! const GITHUB_URL: &str = "https://api.github.com";
//! 
//! struct Github;
//! 
//! #[feign(url = GITHUB_URL)]
//! impl Github {
//!     #[get]
//!     async fn home() -> feignhttp::Result<String> {}
//! 
//!     #[get("/repos/{owner}/{repo}")]
//!     async fn repository(
//!         #[path] owner: &str,
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
//! ### Timeout Configuration
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
//! ## Logs
//! 
//! FeignHttp logs some useful information about requests and responses with the [log](https://docs.rs/log) crate.
//! To enable the log information, specify `log` feature in `Cargo.toml`, then set the log level to debug.
//! 
//! ```toml
//! features = ["log"]
//! ```
//! 
//! ## Optional Features
//!
//! The following features are available. The default features are `reqwest-client`.
//! * **reqwest-client** *(default)*: Use `reqwest` as the HTTP backend.
//! * **isahc-client**: Use `isahc` as the HTTP backend.
//! * **json**: Enable json serialize and deserialize.
//! * **log**: Enable request and response logs.

mod http;
mod error;
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

pub use feignhttp_codegen::*;

pub use crate::http::*;
pub use crate::error::{Result, Error, ErrorKind};
