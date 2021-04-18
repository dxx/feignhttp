use feignhttp::{get, post};

use serde::{Serialize};
use mockito::{mock, Matcher};

#[get("http://localhost:1234/get")]
async fn get() -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_get() {
    let _mock = mock("GET", "/get").create();

    get().await.unwrap();
}


#[post(url = "http://localhost:1234/post")]
async fn post() -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_post() {
    let _mock = mock("POST", "/post").create();

    post().await.unwrap();
}


#[post(url = "http://localhost:1234/post_header")]
async fn post_header(
    #[header] auth: String,
    #[header("name")] username: &str,
) -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_header() {
    let _mock = mock("POST", "/post_header")
        .match_header("auth", "name")
        .match_header("name", "jack")
        .create();

    post_header("name".to_string(), "jack").await.unwrap();
}


#[post(url = "http://localhost:1234/post_query")]
async fn post_query(
    #[param] id: u32,
    #[param("name")] name: String,
) -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_query() {
    let _mock = mock("POST", "/post_query")
        .match_query(Matcher::Regex("id=1".into()))
        .match_query(Matcher::Regex("name=xxx".into()))
        .create();

    post_query(1, "xxx".to_string()).await.unwrap();
}


#[post(url = "http://localhost:1234/post_form")]
async fn post_form(
    #[form] id: i32,
    #[form("name")] name: String,
) -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_send_form() {
    let _mock = mock("POST", "/post_form")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body(r#"id=1&name=xxx"#)
        .create();

    post_form(1, "xxx".to_string()).await.unwrap();
}


#[post(url = "http://localhost:1234/post_text")]
async fn post_text(#[body] text: String) -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_send_text() {
    let _mock = mock("POST", "/post_text")
        .match_header("content-type", "text/plain")
        .match_body(r#"I' m text"#)
        .create();

    post_text("I' m text".to_string()).await.unwrap();
}


#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "http://localhost:1234/post_json")]
async fn post_json(#[body] user: User) -> feignhttp::Result<String> {}

#[async_std::test]
async fn test_send_json() {
    let _mock = mock("POST", "/post_json")
        .match_header("content-type", "application/json")
        .match_body(r#"{"id":1,"name":"jack"}"#)
        .create();

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let _r = post_json(user).await.unwrap();
}


#[get(url = "http://xxx.com", connect_timeout = 3000)]
async fn connect_timeout() -> feignhttp::Result<String> {}

#[async_std::test]
#[should_panic]
async fn test_connect_timeout() {
    connect_timeout().await.unwrap();
}


#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}

#[async_std::test]
#[should_panic]
async fn test_timeout() {
    timeout().await.unwrap();
}
