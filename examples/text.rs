#[allow(dead_code)]
mod support;

use feignhttp::post;
use serde::{Deserialize, Serialize};

use support::*;
use hyper::body::HttpBody;

// Serialization and deserialization
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "http://localhost:8080/create")]
async fn create_user(#[body] json: String) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _server = server::http(8080, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        let body = String::from_utf8(vec).unwrap();
        println!("method: {}", req.method());
        println!("received: {}", body);

        let user: User = serde_json::from_str(&body).unwrap();
        println!("user : {:?}", user);

        hyper::Response::default()
    });


    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let json = serde_json::to_string(&user).unwrap();
    let _r = create_user(json).await?;

    Ok(())
}
