#[allow(dead_code)]
mod support;

use feignhttp::{get, post};
use support::*;

use serde::{Serialize};
use hyper::body::HttpBody;


#[get("http://localhost:8080/get")]
async fn get() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_get() {
    let _server = server::http(8080, move |req| async move {
        assert_eq!(req.method(), "GET");

        hyper::Response::default()
    });

    let _r = get().await.unwrap();
}


#[post(url = "http://localhost:8080/post")]
async fn post() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_post() {
    let _server = server::http(8080, move |req| async move {
        assert_eq!(req.method(), "POST");

        hyper::Response::default()
    });

    let _r = post().await.unwrap();
}


#[post(url = "http://localhost:8080/post_header")]
async fn post_header (
    #[header] auth: String,
    #[header("name")] username: &str)
    -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_header() {
    let _server = server::http(8080, move |req| async move {
        assert_eq!(req.headers()["auth"], "name");
        assert_eq!(req.headers()["name"], "jack");

        hyper::Response::default()
    });

    let _r = post_header("name".to_string(), "jack").await.unwrap();
}


#[post(url = "http://localhost:8080/post_query")]
async fn post_query (
    #[param] id: u32,
    #[param("name")] name: String)
    -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_query() {
    let _server = server::http(8080, move |req| async move {
        assert_eq!("id=1&name=xxx", req.uri().query().unwrap());

        hyper::Response::default()
    });

    let _r = post_query(1, "xxx".to_string()).await.unwrap();
}


#[post(url = "http://localhost:8080/post_text")]
async fn post_text (#[body] text: String) -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_send_text() {
    let _server = server::http(8080, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        assert_eq!("I' m text", String::from_utf8(vec).unwrap());

        hyper::Response::default()
    });

    let _r = post_text("I' m text".to_string()).await.unwrap();
}


#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "http://localhost:8080/post_json")]
async fn post_json (#[body] user: User) -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
async fn test_send_json() {
    let _server = server::http(8080, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        assert_eq!(r#"{"id":1,"name":"jack"}"#, String::from_utf8(vec).unwrap());

        hyper::Response::default()
    });

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let _r = post_json(user).await.unwrap();
}


#[get(url = "http://xxx.com", connect_timeout = 3000)]
async fn connect_timeout() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
#[should_panic]
async fn test_connect_timeout() {
    connect_timeout().await.unwrap();
}


#[get(url = "http://localhost:8080", timeout = 3000)]
async fn timeout() -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::test]
#[should_panic]
async fn test_timeout() {
    let _server = server::http(8080, move |req| async move {
        assert_eq!(req.method(), "GET");

        std::thread::sleep(std::time::Duration::from_millis(5000));

        hyper::Response::default()
    });

    timeout().await.unwrap();
}
