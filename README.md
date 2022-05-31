# FeignHttp

[![crates.io](https://img.shields.io/crates/v/feignhttp.svg)](https://crates.io/crates/feignhttp)
[![Documentation](https://docs.rs/feignhttp/badge.svg)](https://docs.rs/feignhttp)
[![MIT licensed](https://img.shields.io/github/license/dxx/feignhttp.svg?color=blue)](./LICENSE)

FeignHttp is a declarative HTTP client. Based on rust macros.

## Features

* Easy to use
* Asynchronous request
* Configurable timeout settings
* Supports form, plain text and JSON
* Selectable HTTP backends ([reqwest](https://github.com/seanmonstar/reqwest) or [isahc](https://github.com/sagebind/isahc))

## Usage

### Basic

FeignHttp mark macros on asynchronous functions, you need a runtime for support async/await. You can use [async-std](https://github.com/async-rs/async-std) or [tokio](https://github.com/tokio-rs/tokio).

async-std:

```toml
[dependencies]
async-std = { version = "1", features = ["attributes", "tokio1"] }
```

The feature `tokio1` is need when use reqwest as the HTTP backend.

tokio:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Add feignhttp in your `Cargo.toml` and use default feature:

```toml
feignhttp = { version = "0.3" }
```

Then add the following code:

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

The `get` attribute macro specifies get request, `feignhttp::Result<String>` specifies the return result.
It will send get request to `https://api.github.com` and receive a plain text body.

Using non-default HTTP backend:

```toml
feignhttp = { version = "0.3", default-features = false, features = ["isahc-client"] }
```

The `default-features = false` option disable default reqwest.

### JSON

[Serde](https://github.com/serde-rs/serde) is a framework for serializing and deserializing Rust data structures. When use json, you should add serde in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
```

Here is an example of getting json:

```rust
use feignhttp::get;
use serde::Deserialize;

// Deserialize: Specifies deserialization
#[derive(Debug, Deserialize)]
struct IssueItem {
    pub id: u32,
    pub number: u32,
    pub title: String,
    pub url: String,
    pub repository_url: String,
    pub state: String,
    pub body: Option<String>,
}

const GITHUB_URL: &str = "https://api.github.com";

#[get(url = GITHUB_URL, path = "/repos/{owner}/{repo}/issues")]
async fn issues(
    #[path] owner: &str,
    #[path] repo: &str,
    page: u32,
    per_page: u32,
) -> feignhttp::Result<Vec<IssueItem>> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = issues("octocat", "hello-world", 1, 2).await?;
    println!("issues: {:#?}", r);

    Ok(())
}
```

This issues function return `Vec<IssueItem>`, it is deserialized according to the content of the response.

Send a json request:

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let r = post_user(user).await?;
    println!("{}", r);

    Ok(())
}
```

See [here](./examples/json.rs) for a complete example.

### Using structure

Structure is a good way to manage requests. Define a structure and then define a large number of request methods：

```rust
use feignhttp::feign;

const GITHUB_URL: &str = "https://api.github.com";

struct Github;

#[feign(url = GITHUB_URL)]
impl Github {
    #[get]
    async fn home() -> feignhttp::Result<String> {}

    #[get("/repos/{owner}/{repo}")]
    async fn repository(
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String> {}

    // ...
    
    // Structure method still send request
    #[get(path = "/repos/{owner}/{repo}/languages")]
    async fn languages(
        &self,
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String> {}
}
```

See [here](./examples/struct.rs) for a complete example.

### Timeout configuration

If you need to configure the timeout, use `connect_timeout` and `timeout` to specify connect timeout and read timeout.

Connect timeout:

```rust
#[get(url = "http://site_dne.com", connect_timeout = 3000)]
async fn connect_timeout() -> feignhttp::Result<String> {}
```

Read timeout:

```rust
#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}
```

## License

FeignHttp is provided under the MIT license. See [LICENSE](./LICENSE).
