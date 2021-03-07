#[allow(dead_code)]
mod support;

use feignhttp::{get};
use serde::{Deserialize};

use support::*;

const URL: &str = "http://localhost:8080";


#[get(url = URL, path = "/text")]
async fn get_text() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_get_text() {
    let _server = server::http(8080, move |_req| async move {
        hyper::Response::new("Hello, i' m text".into())
    });

    let text = get_text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}


#[derive(Debug, Deserialize)]
struct User {
    code: u32,
    message: String,
}

#[get(url = URL, path = "/json")]
async fn get_json() -> Result<User, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_get_json() {
    let _server = server::http(8080, move |_req| async move {

        hyper::Response::new(r#"{ "code": 200, "message": "success" }"#.into())
    });

    let user = get_json().await.unwrap();

    assert_eq!(r#"User { code: 200, message: "success" }"#, format!("{:?}", user));
}
