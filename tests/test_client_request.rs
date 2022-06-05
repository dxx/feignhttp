#![allow(unused_imports)]

use feignhttp::{map, HttpClient, HttpConfig};

use mockito::{mock, server_address, Matcher};
use serde::Serialize;

#[tokio::test]
async fn test_request() {
    let _mock = mock("GET", "/").create();

    let url = format!("http://{}", server_address());
    let method = "GET";
    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_header() {
    let _mock = mock("GET", "/")
        .match_header("auth", "name_pass")
        .match_header("username", "jack")
        .match_header("pwd", "xxx")
        .create();

    let url = format!("http://{}", server_address());
    let method = "GET";

    let header_map = map!(
        "auth" => "name_pass".to_string(),
        "username" => "jack".to_string(),
        "pwd" => "xxx".to_string());

    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .headers(header_map)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_query() {
    let _mock = mock("GET", "/")
        .match_query(Matcher::Regex("id=1".into()))
        .match_query(Matcher::Regex("name=xxx".into()))
        .match_query(Matcher::Regex("name=xxx2".into()))
        .create();

    let url = format!("http://{}", server_address());
    let method = "GET";

    let query_vec = [
        ("id", "1".to_string()),
        ("name", "xxx".to_string()),
        ("name", "xxx2".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .query(query_vec)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_send_form() {
    let _mock = mock("POST", "/")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(r#"id=1&name=xxx&name=xxx2"#)
        .create();

    let url = format!("http://{}", server_address());
    let method = "POST";

    let form_vec: Vec<(&str, String)> = [
        ("id", "1".to_string()),
        ("name", "xxx".to_string()),
        ("name", "xxx2".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send_form(&form_vec).await.unwrap();
}

#[tokio::test]
async fn test_send_text() {
    let _mock = mock("POST", "/")
        .match_header("content-type", "text/plain")
        .match_body(r#"I' m text"#)
        .create();

    let url = format!("http://{}", server_address());
    let method = "POST";

    let text = r#"I' m text"#;

    let request = HttpClient::builder()
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
        let _mock = mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(r#"{"id":1,"name":"jack"}"#)
            .create();

        let url = format!("http://{}", server_address());
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

        let request = HttpClient::builder()
            .url(&url)
            .method(method)
            .build()
            .unwrap();
        request.send_json(&user).await.unwrap();
    }
}

#[tokio::test]
async fn test_send_vec() {
    let _mock = mock("POST", "/")
        .match_header("content-type", "application/octet-stream")
        .match_body(r#"aaa"#)
        .create();

    let url = format!("http://{}", server_address());
    let method = "POST";

    let vec = vec![97, 97, 97];

    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send_vec(vec).await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_connect_timeout() {
    let url = "http://site_dne.com";
    let method = "GET";
    let config = HttpConfig {
        connect_timeout: Some(3000), // 3000 millisecond.
        timeout: None,
    };
    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .config(config)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_timeout() {
    let url = "https://httpbin.org/delay/5".to_string();
    let method = "GET";
    let config = HttpConfig {
        connect_timeout: None,
        timeout: Some(3000), // 3000 millisecond.
    };
    let request = HttpClient::builder()
        .url(&url)
        .method(method)
        .config(config)
        .build()
        .unwrap();
    request.send().await.unwrap();
}
