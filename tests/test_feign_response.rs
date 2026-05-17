#![allow(dead_code)]

use feignhttp::get;

use mockito::{Server, ServerOpts};
use serde::Deserialize;

const TEXT_URL: &str = "http://localhost:1250";

#[get(url = TEXT_URL, path = "/text")]
async fn get_text() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_get_text() {
    let opts = ServerOpts {
        port: 1250,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("GET", "/text")
        .with_body("Hello, i' m text")
        .create_async()
        .await;

    let text = get_text().await.unwrap();

    assert_eq!("Hello, i' m text", text);
}

const JSON_URL: &str = "http://localhost:1251";

#[derive(Debug, Deserialize)]
struct User {
    code: u32,
    message: String,
}

#[cfg(feature = "json")]
#[get(url = JSON_URL, path = "/json")]
async fn get_json() -> feignhttp::Result<User> {}

#[tokio::test]
async fn test_get_json() {
    #[cfg(feature = "json")]
    {
        let opts = ServerOpts {
            port: 1251,
            ..Default::default()
        };
        let mut server = Server::new_with_opts_async(opts).await;
        let _mock = server
            .mock("GET", "/json")
            .with_body("{ \"code\": 200, \"message\": \"success\" }")
            .create_async()
            .await;

        let user = get_json().await.unwrap();

        assert_eq!(
            "User { code: 200, message: \"success\" }",
            format!("{:?}", user)
        );
    }
}

const VEC_URL: &str = "http://localhost:1252";

#[get(url = VEC_URL, path = "/vec")]
async fn get_data() -> feignhttp::Result<Vec<u8>> {}

#[tokio::test]
async fn test_get_vec() {
    let opts = ServerOpts {
        port: 1252,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock = server
        .mock("GET", "/vec")
        .with_header("content-type", "application/octet-stream")
        .with_body("aaa")
        .create_async()
        .await;

    let vec = get_data().await.unwrap();

    assert_eq!(vec![97, 97, 97], vec);
}
