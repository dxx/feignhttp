#[allow(dead_code)]
mod support;

use feignhttp::{get};
use serde::{Deserialize};

use support::*;

const TEXT_URL: &str = "http://localhost:8080";


#[get(url = TEXT_URL, path = "/text")]
async fn get_text() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_get_text() {
    let _server = server::http(8080, move |_req| async move {
        hyper::Response::new("Hello, i' m text".into())
    });

    let text = get_text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}


const JSON_URL: &str = "http://localhost:8081";

#[derive(Debug, Deserialize)]
struct User {
    code: u32,
    message: String,
}

#[get(url = JSON_URL, path = "/json")]
async fn get_json() -> feignhttp::Result<User> {}

#[tokio::test]
async fn test_get_json() {
    let _server = server::http(8081, move |_req| async move {

        hyper::Response::new(r#"{ "code": 200, "message": "success" }"#.into())
    });

    let user = get_json().await.unwrap();

    assert_eq!(r#"User { code: 200, message: "success" }"#, format!("{:?}", user));
}
