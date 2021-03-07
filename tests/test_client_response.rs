mod support;

use feignhttp::HttpClient;
use serde::{Deserialize};

use support::*;

#[tokio::test]
async fn test_response() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.method(), "GET");

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    let response = request.send().await.unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_get_text() {
    let server = server::http(0, move |_req| async move {
        hyper::Response::new("Hello, i' m text".into())
    });

    let url = format!("http://{}/text", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    let response = request.send().await.unwrap();
    let text = response.text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}

#[tokio::test]
async fn test_get_json() {
    let server = server::http(0, move |_req| async move {

        hyper::Response::new(r#"{ "code": 200, "message": "success" }"#.into())
    });

    #[derive(Debug, Deserialize)]
    struct User {
        code: u32,
        message: String,
    }

    let url = format!("http://{}/json", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    let response = request.send().await.unwrap();
    let user: User = response.json().await.unwrap();

    assert_eq!(r#"User { code: 200, message: "success" }"#, format!("{:?}", user));
}

#[tokio::test]
#[should_panic]
async fn test_client_error() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.method(), "GET");

        hyper::Response::builder().status(hyper::StatusCode::NOT_FOUND).body("".into()).unwrap()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    let _response = request.send().await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_server_error() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.method(), "GET");

        hyper::Response::builder().status(hyper::StatusCode::SERVICE_UNAVAILABLE).body("".into()).unwrap()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    let _response = request.send().await.unwrap();
}
