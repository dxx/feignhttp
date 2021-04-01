use feignhttp::{HttpClient, HttpResponse};

use serde::{Deserialize};
use mockito::{mock, server_address};

#[async_std::test]
async fn test_response() {
    let _mock = mock("GET", "/").with_status(200).create();

    let url = format!("http://{}", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    let response = request.send().await.unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[async_std::test]
async fn test_get_text() {
    let _mock = mock("GET", "/text")
        .with_body("Hello, i' m text")
        .create();

    let url = format!("http://{}/text", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    let response = request.send().await.unwrap();
    let text = response.text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}

#[async_std::test]
async fn test_get_json() {
    let _mock = mock("GET", "/json")
        .with_body(r#"{ "code": 200, "message": "success" }"#)
        .create();

    #[derive(Debug, Deserialize)]
    struct User {
        code: u32,
        message: String,
    }

    let url = format!("http://{}/json", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    let response = request.send().await.unwrap();
    let user: User = response.json().await.unwrap();

    assert_eq!(r#"User { code: 200, message: "success" }"#, format!("{:?}", user));
}

#[async_std::test]
#[should_panic]
async fn test_client_error() {
    let _mock = mock("GET", "/")
        .with_status(404)
        .create();

    let url = format!("http://{}", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    let _response = request.send().await.unwrap();
}

#[async_std::test]
#[should_panic]
async fn test_server_error() {
    let _mock = mock("GET", "/")
        .with_status(503)
        .create();

    let url = format!("http://{}", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    let _response = request.send().await.unwrap();
}
