# FeignHttp

[![MIT licensed](https://img.shields.io/github/license/code-mcx/feignhttp.svg?color=blue)](./LICENSE)

FeignHttp is a declarative HTTP client. Based on rust macros.

## Features

* Easy to use
* Asynchronous request
* Supports `json` and `plain text`
* [Reqwest](https://github.com/seanmonstar/reqwest) of internal use

## Usage

FeignHttp mark macros on asynchronous functions, you need to add [tokio](https://github.com/tokio-rs/tokio) in your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
feignhttp = { version = "0.0.1" }
```

Then add the following code:

```rust
use feignhttp::get;

#[get("https://api.github.com")]
async fn github() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = github().await?;
    println!("result: {}", r);

    Ok(())
}
```

## License

FeignHttp is provided under the MIT license. See [LICENSE](./LICENSE).

