use feignhttp::{get, post};

use mockito::{Matcher, Server, ServerOpts};
use serde::Serialize;

#[get("http://localhost:1240/get")]
async fn get() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_get() {
    let opts = ServerOpts {
        port: 1240,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server.mock("GET", "/get").create_async().await;

    get().await.unwrap();
}

#[post(url = "http://localhost:1241/post")]
async fn post() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_post() {
    let opts = ServerOpts {
        port: 1241,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server.mock("POST", "/post").create_async().await;

    post().await.unwrap();
}

#[post(
    url = "http://localhost:1242/post_header",
    headers = "auth: password; pwd: {pwd}"
)]
async fn post_header(
    #[header] auth: String,
    #[header("name")] username: &str,
    #[param] pwd: &str,
) -> feignhttp::Result<String> {
}

#[tokio::test]
async fn test_header() {
    let opts = ServerOpts {
        port: 1242,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("POST", "/post_header")
        .match_header("auth", "name")
        .match_header("name", "jack")
        .match_header("pwd", "MTIzNDU2")
        .create_async()
        .await;

    post_header("name".to_string(), "jack", "MTIzNDU2")
        .await
        .unwrap();
}

#[post(url = "http://localhost:1243/post_query")]
async fn post_query(#[query] id: u32, #[query("name")] name: String) -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_query() {
    let opts = ServerOpts {
        port: 1243,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("POST", "/post_query")
        .match_query(Matcher::Regex("id=1".into()))
        .match_query(Matcher::Regex("name=xxx".into()))
        .create_async()
        .await;

    post_query(1, "xxx".to_string()).await.unwrap();
}

#[post(url = "http://localhost:1244/post_form")]
async fn post_form(#[form] id: i32, #[form("name")] name: String) -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_send_form() {
    let opts = ServerOpts {
        port: 1244,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("POST", "/post_form")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("id=1&name=xxx")
        .create_async()
        .await;

    post_form(1, "xxx".to_string()).await.unwrap();
}

#[post(url = "http://localhost:1245/post_text")]
async fn post_text(#[body] text: String) -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_send_text() {
    let opts = ServerOpts {
        port: 1245,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("POST", "/post_text")
        .match_header("content-type", "text/plain")
        .match_body("I' m text")
        .create_async()
        .await;

    post_text("I' m text".to_string()).await.unwrap();
}

#[allow(dead_code)]
#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[cfg(feature = "json")]
#[post(url = "http://localhost:1246/post_json")]
async fn post_json(#[body] user: User) -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_send_json() {
    #[cfg(feature = "json")]
    {
        let opts = ServerOpts {
            port: 1246,
            ..Default::default()
        };
        let mut server = Server::new_with_opts_async(opts).await;
        let _mock = server
            .mock("POST", "/post_json")
            .match_header("content-type", "application/json")
            .match_body("{\"id\":1,\"name\":\"jack\"}")
            .create_async()
            .await;

        let user = User {
            id: 1,
            name: "jack".to_string(),
        };
        let _r = post_json(user).await.unwrap();
    }
}

#[post(url = "http://localhost:1247/post_vec")]
async fn post_data(#[body] data: Vec<u8>) -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_send_vec() {
    let opts = ServerOpts {
        port: 1247,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("POST", "/post_vec")
        .match_header("content-type", "application/octet-stream")
        .match_body("aaa")
        .create_async()
        .await;

    post_data(vec![97, 97, 97]).await.unwrap();
}
