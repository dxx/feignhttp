mod support;

use feignhttp::{HttpClient, map};
use support::*;

use serde::{Serialize};
use hyper::body::HttpBody;

#[tokio::test]
async fn test_request() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.method(), "GET");

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";
    let request = HttpClient::new().build_request(&url, method);
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_header() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.headers()["auth"], "name_pass");
        assert_eq!(req.headers()["username"], "jack");
        assert_eq!(req.headers()["pwd"], "xxx");

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";

    let header_map = map!(
        "auth" => "name_pass".to_string(),
        "username" => "jack".to_string(),
        "pwd" => "xxx".to_string());

    let request  = HttpClient::new()
        .build_request(&url, method)
        .headers(header_map);
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_query() {
    let server = server::http(0, move |req| async move {
        assert_eq!("id=1&name=xxx&name=xxx2", req.uri().query().unwrap());

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "get";

    let query_vec = [
        ("id", "1".to_string()),
        ("name", "xxx".to_string()),
        ("name", "xxx2".to_string()),
    ].iter().cloned().collect();

    let request  = HttpClient::new().build_request(&url, method).query(&query_vec);
    request.send().await.unwrap();
}

#[tokio::test]
async fn test_send_text() {
    let server = server::http(0, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        assert_eq!("I' m text", String::from_utf8(vec).unwrap());

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "post";

    let text = r#"I' m text"#;
    let request = HttpClient::new().build_request(&url, method);
    request.send_text(text.to_string()).await.unwrap();
}

#[tokio::test]
async fn test_send_json() {
    let server = server::http(0, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        assert_eq!(r#"{"id":1,"name":"jack"}"#, String::from_utf8(vec).unwrap());

        hyper::Response::default()
    });

    let url = format!("http://{}", server.addr());
    let method = "post";

    #[derive(Serialize)]
    struct User {
        id: i32,
        name: String,
    }

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };

    let request = HttpClient::new().build_request(&url, method);
    request.send_json(&user).await.unwrap();
}
