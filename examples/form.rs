use feignhttp::post;

use serde::{Serialize};

// Serialize: Specifies serialization
#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "https://httpbin.org/anything")]
async fn post_user(#[form] id: i32, #[form] name: &str) -> feignhttp::Result<String> {}

#[post(url = "https://httpbin.org/anything")]
async fn post_user2(#[form] user: User) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "feignhttp=debug");
    env_logger::init();

    let r = post_user(1, "jack").await?;
    println!("{}", r);

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let r = post_user2(user).await?;
    println!("{}", r);

    Ok(())
}
