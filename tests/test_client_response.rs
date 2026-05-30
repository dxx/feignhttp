#![allow(dead_code)]
#![allow(unused_imports)]

use feignhttp::{HttpClient, HttpRequest, HttpResponse, RequestBuilder};

use mockito::Server;
use serde::Deserialize;

#[tokio::test]
async fn test_response() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    let response = request.send().await.unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_get_text() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/text")
        .with_body("Hello, i' m text")
        .create_async()
        .await;

    let url = format!("http://{}/text", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    let response = request.send().await.unwrap();
    let text = response.text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}

#[tokio::test]
async fn test_get_json() {
    #[cfg(feature = "json")]
    {
        let mut server = Server::new_async().await;
        let _mock = server
            .mock("GET", "/json")
            .with_body(r#"{ "code": 200, "message": "success" }"#)
            .create_async()
            .await;

        #[derive(Debug, Deserialize)]
        struct User {
            code: u32,
            message: String,
        }

        let url = format!("http://{}/json", server.host_with_port());
        let method = "GET";
        let request = RequestBuilder::new(HttpClient::new().unwrap())
            .url(&url)
            .method(method)
            .build()
            .unwrap();
        let response = request.send().await.unwrap();
        let user: User = response.json().await.unwrap();

        assert_eq!(
            r#"User { code: 200, message: "success" }"#,
            format!("{:?}", user)
        );
    }
}

#[tokio::test]
async fn test_get_vec() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/vec")
        .with_header("content-type", "application/octet-stream")
        .with_body(r#"aaa"#)
        .create_async()
        .await;

    let url = format!("http://{}/vec", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    let response = request.send().await.unwrap();
    let vec = response.vec().await.unwrap();

    assert_eq!(vec![97, 97, 97], vec);
}

#[tokio::test]
#[should_panic]
async fn test_client_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/")
        .with_status(404)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    let _response = request.send().await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_server_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/")
        .with_status(503)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    let _response = request.send().await.unwrap();
}
