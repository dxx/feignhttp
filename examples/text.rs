use feignhttp::post;

use serde::{Deserialize, Serialize};

// Serialization and deserialization
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "https://httpbin.org/anything")]
async fn post_user(#[body] json: String) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    post_user(json).await?;

    Ok(())
}
