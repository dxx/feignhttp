#![allow(unused_imports)]

use feignhttp::{HttpClient, HttpRequest, RequestBuilder, map};
use serde::Serialize;

use mockito::{Matcher, Server};

#[tokio::test]
async fn test_request() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/").create_async().await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_header() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/")
        .match_header("auth", "name_pass")
        .match_header("username", "jack")
        .match_header("pwd", "xxx")
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";

    let header_map = map!(
        "auth".into() => "name_pass".to_string(),
        "username".into() => "jack".to_string(),
        "pwd".into() => "xxx".to_string());

    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .headers(header_map)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_query() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/")
        .match_query(Matcher::Regex("id=1".into()))
        .match_query(Matcher::Regex("name=xxx".into()))
        .match_query(Matcher::Regex("name=xxx2".into()))
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "GET";

    let query_vec = [
        ("id", "1".to_string()),
        ("name", "xxx".to_string()),
        ("name", "xxx2".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .query(query_vec)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_send_form() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(r#"id=1&name=xxx&name=xxx2"#)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "POST";

    let form_vec: Vec<(&str, String)> = [
        ("id", "1".to_string()),
        ("name", "xxx".to_string()),
        ("name", "xxx2".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send_form(&form_vec).await.unwrap();
}

#[tokio::test]
async fn test_send_text() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/")
        .match_header("content-type", "text/plain")
        .match_body(r#"I' m text"#)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "POST";

    let text = r#"I' m text"#;

    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send_text(text.to_string()).await.unwrap();
}

#[tokio::test]
async fn test_send_json() {
    #[cfg(feature = "json")]
    {
        let mut server = Server::new_async().await;
        let _mock = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(r#"{"id":1,"name":"jack"}"#)
            .create_async()
            .await;

        let url = format!("http://{}", server.host_with_port());
        let method = "POST";

        #[derive(Serialize)]
        struct User {
            id: i32,
            name: String,
        }

        let user = User {
            id: 1,
            name: "jack".to_string(),
        };

        let request = RequestBuilder::new(HttpClient::new().unwrap())
            .url(&url)
            .method(method)
            .build()
            .unwrap();
        request.send_json(&user).await.unwrap();
    }
}

#[tokio::test]
async fn test_send_vec() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/")
        .match_header("content-type", "application/octet-stream")
        .match_body(r#"aaa"#)
        .create_async()
        .await;

    let url = format!("http://{}", server.host_with_port());
    let method = "POST";

    let vec = vec![97, 97, 97];

    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send_vec(vec).await.unwrap();
}
