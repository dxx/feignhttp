# FeignHTTP

FeignHTTP is a declarative HTTP client based on Rust macros.

## Features

- Easy to use
- Asynchronous request
- Supports plain text, form, multipart and JSON
- Configurable timeout settings
- Friendly error handling
- Selectable HTTP backends ([reqwest](https://docs.rs/reqwest), [reqwest-middleware](https://docs.rs/reqwest-middleware) or [isahc](https://docs.rs/isahc))

## Table of Contents

- [Usage](#usage)
- [Making a POST request](#making-a-post-request)
- [Paths](#paths)
- [URL](#url)
- [Query Parameters](#query-parameters)
- [Headers](#headers)
- [Form](#form)
- [Multipart](#multipart)
- [JSON](#json)
- [Using Trait](#using-trait)
- [Timeout](#timeout)
- [Params](#params)
- [Configuration](#configuration)
- [Customize Client](#customize-client)
- [Error Handling](#error-handling)
- [Debug Logs](#debug-logs)
- [Optional Features](#optional-features)

## Usage

FeignHTTP uses macros on asynchronous functions. You need a runtime for async/await support, such as [tokio](https://docs.rs/tokio).

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Add `feignhttp` in your `Cargo.toml`:

```toml
feignhttp = { version = "0.6" }
```

Basic example:

```rust
use feignhttp::get;

#[get("https://api.github.com")]
async fn github() -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = github().await?;
    println!("result: {}", r);
    Ok(())
}
```

Using non-default HTTP backend:

```toml
feignhttp = { version = "0.6", default-features = false, features = ["isahc-client"] }
```

## Making a POST request

Use the `post` attribute macro and `#[body]` for request body:

```rust
use feignhttp::post;

#[post("https://httpbin.org/anything")]
async fn post_data(#[body] text: String) -> feignhttp::Result<String> {}
```

`String` and `&str` are sent as plain text with `content-type: text/plain`.

## Paths

Use `path` to specify path values:

```rust
use feignhttp::get;

#[get("https://api.github.com/repos/{owner}/{repo}")]
async fn repository(
    #[path("owner")] user: &str,
    #[path] repo: String,
) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = repository("dxx", "feignhttp".to_string()).await?;
    println!("repository result: {}", r);
    Ok(())
}
```

## URL

Use constants for URLs:

```rust
use feignhttp::get;

const GITHUB_URL: &str = "https://api.github.com";

#[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<String> {}
```

## Query Parameters

Use `query` for query parameters:

```rust
use feignhttp::get;

#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32,
) -> feignhttp::Result<String> {}
```

**Note**: Parameters without any attribute are query parameters by default.

## Headers

Use `header` for request headers:

```rust
use feignhttp::get;

#[get("https://httpbin.org/headers")]
async fn headers(
    #[header] accept: &str,
) -> feignhttp::Result<String> {}
```

Or use `headers` metadata:

```rust
#[get("https://httpbin.org/headers", headers = "key1: value1; key2: value2")]
async fn headers() -> feignhttp::Result<String> {}
```

## Form

Use `form` for form parameters:

```rust
use feignhttp::post;

#[post(url = "https://httpbin.org/anything")]
async fn post_user(
    #[form] id: i32,
    #[form] name: &str,
) -> feignhttp::Result<String> {}
```

Automatically adds `content-type: application/x-www-form-urlencoded`.

## Multipart

Use `part` for form fields and `file` for file uploads:

```toml
feignhttp = { version = "0.6", features = ["reqwest-multipart"] }
```

```rust
use feignhttp::post;
use std::path::PathBuf;

#[post("https://httpbin.org/post")]
async fn upload_file(
    #[file("file")] file: PathBuf,
    #[part("name")] name: &str,
) -> feignhttp::Result<String> {}
```

Specify `content_type` and `filename`:

```rust
#[post("https://httpbin.org/post")]
async fn upload_file(
    #[file("file", content_type = "image/png", filename = "custom.png")] file: PathBuf,
    #[part("name")] name: &str,
) -> feignhttp::Result<String> {}
```

Supported file types: `PathBuf`, `std::fs::File`, `Vec<u8>`.

## JSON

Requires [serde](https://docs.rs/serde):

```toml
serde = { version = "1", features = ["derive"] }
feignhttp = { version = "0.6", features = ["reqwest-json"] }
```

Deserialize JSON:

```rust
use feignhttp::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IssueItem {
    pub id: u32,
    pub number: u32,
    pub title: String,
}

const GITHUB_URL: &str = "https://api.github.com";

#[get(url = GITHUB_URL, path = "/repos/{owner}/{repo}/issues")]
async fn issues(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<Vec<IssueItem>> {}
```

Send JSON request:

```rust
use feignhttp::post;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "https://httpbin.org/anything")]
async fn post_user(#[body] user: User) -> feignhttp::Result<String> {}
```

Automatically adds `content-type: application/json`.

## Using Trait

Traits are a good way to manage multiple requests:

```rust
use feignhttp::{Context, FeignClientBuilder, feign};

#[derive(Context)]
struct GithubContext {
    #[url_path("owner")]
    user: &'static str,
    #[url_path]
    repo: &'static str,
    #[param]
    accept: &'static str,
}

#[feign(
    url = "https://api.github.com/repos/{owner}/{repo}",
    headers = "Accept: {accept}"
)]
pub trait Github {
    #[get]
    async fn home(&self) -> feignhttp::Result<String>;

    #[get(path = "", headers = "Accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = GithubContext {
        user: "dxx",
        repo: "feignhttp",
        accept: "*/*",
    };

    let github = Github::builder().context(context).build()?;
    let r = github.home().await?;
    println!("github result: {}", r);
    Ok(())
}
```

## Timeout

Configure timeout with `timeout`:

```rust
use feignhttp::get;

#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}
```

## Params

Use `param` for dynamic values:

```rust
use feignhttp::get;

#[get(url = "https://httpbin.org/delay/5", timeout = "{time}")]
async fn timeout(#[param] time: u16) -> feignhttp::Result<String> {}
```

## Configuration

Use `ClientConfig` for trait-based clients:

```rust
use feignhttp::{ClientConfig, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com")]
pub trait GitHub {
    #[get("/repos/{owner}/{repo}")]
    async fn repository(
        &self,
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        connect_timeout: Some(5000),
        timeout: Some(10000),
        ..Default::default()
    };

    let github = GitHub::builder().config(config).build()?;
    let r = github.repository("dxx", "feignhttp").await?;
    println!("repository: {}", r);
    Ok(())
}
```

## Customize Client

Customize the HTTP client:

```rust
use feignhttp::{ClientWrapper, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com")]
pub trait GitHub {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Feign HTTP")
        .build()?;

    let client_wrapper = ClientWrapper::with_client(client)?;
    let github = GitHub::builder().build_with_client(client_wrapper)?;
    let r = github.user("dxx").await?;
    println!("{}", r);
    Ok(())
}
```

## Error Handling

Use `feignhttp::Result` and handle errors:

```rust
use feignhttp::{get, ErrorKind};

#[get(url = "https://httpbin.org/123")]
async fn status_error() -> feignhttp::Result<()> {}

#[tokio::main]
async fn main() {
    match status_error().await {
        Err(err) => {
            if err.is_status_error() {
                println!("status_error: {}", err);
            }
            if let ErrorKind::Status(status) = err.error_kind() {
                println!("status code: {}", status.as_u16());
            }
        }
        _ => {}
    }
}
```

## Debug Logs

Enable logs with `log` feature:

```toml
feignhttp = { version = "0.6", features = ["log"] }
```

Then set log level to debug.

## Optional Features

Default features: `reqwest-client`

- **reqwest-client**: Use `reqwest` as the HTTP backend
- **reqwest-middleware-client**: Use `reqwest-middleware` as the HTTP backend
- **isahc-client**: Use `isahc` as the HTTP backend
- **reqwest-json**: Enable json for `reqwest` backend
- **reqwest-middleware-json**: Enable json for `reqwest-middleware` backend
- **isahc-json**: Enable json for `isahc` backend
- **reqwest-multipart**: Enable multipart for `reqwest` backend
- **reqwest-middleware-multipart**: Enable multipart for `reqwest-middleware` backend
- **isahc-multipart**: Enable multipart for `isahc` backend
- **json**: Enable json serialization and deserialization
- **multipart**: Enable multipart support (required for file uploads)
- **log**: Enable request and response logs