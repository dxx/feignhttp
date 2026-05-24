# FeignHTTP

FeignHTTP 是一个基于 Rust 宏的声明式 HTTP 客户端。

## 特性

- 易于使用
- 异步请求
- 支持纯文本、表单、多类型表单和 JSON
- 可配置超时设置
- 友好的错误处理
- 可选的 HTTP 后端 ([reqwest](https://docs.rs/reqwest), [reqwest-middleware](https://docs.rs/reqwest-middleware) 或 [isahc](https://docs.rs/isahc))

## 目录

- [快速开始](#快速开始)
- [发送 POST 请求](#发送-post-请求)
- [路径参数](#路径参数)
- [URL](#url)
- [查询参数](#查询参数)
- [请求头](#请求头)
- [表单](#表单)
- [多类型表单](#多类型表单)
- [JSON](#json)
- [使用 Trait](#使用-trait)
- [超时配置](#超时配置)
- [参数替换](#参数替换)
- [客户端配置](#客户端配置)
- [自定义客户端](#自定义客户端)
- [错误处理](#错误处理)
- [调试日志](#调试日志)
- [可选特性](#可选特性)

## 快速开始

FeignHTTP 通过宏标记异步函数，需要运行时支持 async/await，推荐使用 [tokio](https://docs.rs/tokio)。

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

在 `Cargo.toml` 中添加 `feignhttp`:

```toml
feignhttp = { version = "0.6" }
```

基本示例:

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

使用非默认 HTTP 后端:

```toml
feignhttp = { version = "0.6", default-features = false, features = ["isahc-client"] }
```

## 发送 POST 请求

使用 `post` 属性宏和 `#[body]` 指定请求体:

```rust
use feignhttp::post;

#[post("https://httpbin.org/anything")]
async fn post_data(#[body] text: String) -> feignhttp::Result<String> {}
```

`String` 和 `&str` 将作为纯文本发送，自动添加 `content-type: text/plain`。

## 路径参数

使用 `path` 指定路径参数:

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

使用常量管理 URL:

```rust
use feignhttp::get;

const GITHUB_URL: &str = "https://api.github.com";

#[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<String> {}
```

## 查询参数

使用 `query` 指定查询参数:

```rust
use feignhttp::get;

#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32,
) -> feignhttp::Result<String> {}
```

**注意**: 没有属性标记的参数默认为查询参数。

## 请求头

使用 `header` 指定请求头:

```rust
use feignhttp::get;

#[get("https://httpbin.org/headers")]
async fn headers(
    #[header] accept: &str,
) -> feignhttp::Result<String> {}
```

或使用 `headers` 元数据:

```rust
#[get("https://httpbin.org/headers", headers = "key1: value1; key2: value2")]
async fn headers() -> feignhttp::Result<String> {}
```

## 表单

使用 `form` 指定表单参数:

```rust
use feignhttp::post;

#[post(url = "https://httpbin.org/anything")]
async fn post_user(
    #[form] id: i32,
    #[form] name: &str,
) -> feignhttp::Result<String> {}
```

自动添加 `content-type: application/x-www-form-urlencoded`。

## 多类型表单

使用 `part` 指定表单字段，`file` 指定文件上传:

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

指定 `content_type` 和 `filename`:

```rust
#[post("https://httpbin.org/post")]
async fn upload_file(
    #[file("file", content_type = "image/png", filename = "custom.png")] file: PathBuf,
    #[part("name")] name: &str,
) -> feignhttp::Result<String> {}
```

支持的文件类型: `PathBuf`、`std::fs::File`、`Vec<u8>`。

## JSON

需要 [serde](https://docs.rs/serde):

```toml
serde = { version = "1", features = ["derive"] }
feignhttp = { version = "0.6", features = ["reqwest-json"] }
```

反序列化 JSON:

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

发送 JSON 请求:

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

自动添加 `content-type: application/json`。

## 使用 Trait

Trait 是管理多个请求的好方法:

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

## 超时配置

使用 `timeout` 配置超时:

```rust
use feignhttp::get;

#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}
```

## 参数替换

使用 `param` 动态替换值:

```rust
use feignhttp::get;

#[get(url = "https://httpbin.org/delay/5", timeout = "{time}")]
async fn timeout(#[param] time: u16) -> feignhttp::Result<String> {}
```

## 客户端配置

使用 `ClientConfig` 配置 trait 客户端:

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

## 自定义客户端

自定义 HTTP 客户端:

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

## 错误处理

使用 `feignhttp::Result` 处理错误:

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

## 调试日志

启用 `log` 特性:

```toml
feignhttp = { version = "0.6", features = ["log"] }
```

然后设置日志级别为 debug。

## 可选特性

默认特性: `reqwest-client`

- **reqwest-client**: 使用 `reqwest` 作为 HTTP 后端
- **reqwest-middleware-client**: 使用 `reqwest-middleware` 作为 HTTP 后端
- **isahc-client**: 使用 `isahc` 作为 HTTP 后端
- **reqwest-json**: 为 `reqwest` 后端启用 JSON
- **reqwest-middleware-json**: 为 `reqwest-middleware` 后端启用 JSON
- **isahc-json**: 为 `isahc` 后端启用 JSON
- **reqwest-multipart**: 为 `reqwest` 后端启用多类型表单
- **reqwest-middleware-multipart**: 为 `reqwest-middleware` 后端启用多类型表单
- **isahc-multipart**: 为 `isahc` 后端启用多类型表单
- **json**: 启用 JSON 序列化/反序列化
- **multipart**: 启用多类型表单支持（文件上传必需）
- **log**: 启用请求和响应日志