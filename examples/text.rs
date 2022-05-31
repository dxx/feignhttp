use feignhttp::post;

use serde::{Serialize};

// Serialize: Specifies serialization
#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "https://httpbin.org/anything")]
async fn post_user(#[body] json: String) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "feignhttp=debug");
    env_logger::init();

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    let r = post_user(json).await?;
    println!("{}", r);

    Ok(())
}
