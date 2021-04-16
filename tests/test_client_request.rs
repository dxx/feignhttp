use feignhttp::{HttpClient, HttpConfig, HttpRequest, map};

use mockito::{mock, server_address, Matcher};
use serde::{Serialize};

#[async_std::test]
async fn test_request() {
    let _mock = mock("GET", "/").create();

    let url = format!("http://{}", server_address());
    let method = "GET";
    let request = HttpClient::default_request(&url, method);
    request.send().await.unwrap();
}

#[async_std::test]
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

    let request = HttpClient::default_request(&url, method)
        .headers(header_map);
    request.send().await.unwrap();
}

#[async_std::test]
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
    ].iter().cloned().collect();

    let request = HttpClient::default_request(&url, method).query(&query_vec);
    request.send().await.unwrap();
}

#[async_std::test]
async fn test_send_text() {
    let _mock = mock("POST", "/")
        .match_header("content-type", "text/plain")
        .match_body(r#"I' m text"#)
        .create();

    let url = format!("http://{}", server_address());
    let method = "POST";

    let text = r#"I' m text"#;
    let request = HttpClient::default_request(&url, method);
    request.send_text(text.to_string()).await.unwrap();
}

#[async_std::test]
async fn test_send_json() {
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

    let request = HttpClient::default_request(&url, method);
    request.send_json(&user).await.unwrap();
}

#[async_std::test]
#[should_panic]
async fn test_connect_timeout() {
    let url = "http://xxx.com";
    let method = "GET";
    let config = HttpConfig{
        connect_timeout: Some(3000), // 3000 millisecond
        timeout: None,
    };
    let request = HttpClient::configure_request(&url, method, config);
    request.send().await.unwrap();
}

#[async_std::test]
#[should_panic]
async fn test_timeout() {
    let url = "https://httpbin.org/delay/5".to_string();
    let method = "GET";
    let config = HttpConfig{
        connect_timeout: None,
        timeout: Some(3000), // 3000 millisecond
    };
    let request = HttpClient::configure_request(&url, method, config);
    request.send().await.unwrap();
}
