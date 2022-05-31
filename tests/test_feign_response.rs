#![allow(dead_code)]

use feignhttp::{get};

use serde::{Deserialize};
use mockito::mock;

const TEXT_URL: &str = "http://localhost:1234";

#[get(url = TEXT_URL, path = "/text")]
async fn get_text() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_get_text() {
    let _mock = mock("GET", "/text")
        .with_body("Hello, i' m text")
        .create();

    let text = get_text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}


const JSON_URL: &str = "http://localhost:1234";

#[derive(Debug, Deserialize)]
struct User {
    code: u32,
    message: String,
}

#[get(url = JSON_URL, path = "/json")]
async fn get_json() -> feignhttp::Result<User> {}

#[tokio::test]
async fn test_get_json() {
    let _mock = mock("GET", "/json")
        .with_body(r#"{ "code": 200, "message": "success" }"#)
        .create();

    let user = get_json().await.unwrap();

    assert_eq!(r#"User { code: 200, message: "success" }"#, format!("{:?}", user));
}
