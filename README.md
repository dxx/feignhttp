# FeignHTTP

[![crates.io](https://img.shields.io/crates/v/feignhttp.svg)](https://crates.io/crates/feignhttp)
[![Documentation](https://docs.rs/feignhttp/badge.svg)](https://docs.rs/feignhttp)
[![MIT licensed](https://img.shields.io/github/license/dxx/feignhttp.svg?color=blue)](./LICENSE)

FeignHTTP is a declarative HTTP client. Based on rust macros.

## Features

* Easy to use
* Asynchronous request
* Configurable timeout settings
* Supports form, plain text and JSON
* Selectable HTTP backends ([reqwest](https://github.com/seanmonstar/reqwest) or [isahc](https://github.com/sagebind/isahc))

## Usage

FeignHTTP mark macros on asynchronous functions, you need a runtime for support async/await. You can use [async-std](https://github.com/async-rs/async-std) or [tokio](https://github.com/tokio-rs/tokio).

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

Add `feignhttp` in your `Cargo.toml` and use default feature:

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

For more examples, click [here](./examples).

## Documentation

Read the [documentation](https://docs.rs/feignhttp) for more details.

## License

FeignHTTP is provided under the MIT license. See [LICENSE](./LICENSE).
