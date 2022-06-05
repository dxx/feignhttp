#![allow(unused_imports)]

use feignhttp::post;

use serde::Serialize;

#[derive(Serialize)]
struct Data {
    id: i32,
    name: String,
}

#[cfg(feature = "json")]
#[post("https://httpbin.org/anything")]
async fn anything(#[body] data: Data) -> feignhttp::Result<String> {}

// Specify features = ["log"] in Cargo.toml to enable log feature.
// cargo run --example log --features log.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "feignhttp=debug");
    env_logger::init();

    #[cfg(feature = "json")]
    {
        let data = Data {
            id: 1,
            name: "test".to_string(),
        };
    
        let r = anything(data).await?;
        println!("anything result: {}", r);
    }

    Ok(())
}
