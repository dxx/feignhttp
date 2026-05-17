use feignhttp::{Feign, feign};

use mockito::{Matcher, Server, ServerOpts};
use std::sync::Mutex;

static PORT_LOCK: Mutex<()> = Mutex::new(());

const URL: &str = "https://api.github.com";

#[derive(Feign)]
pub struct Feign;

#[feign(url = URL)]
impl Feign {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String> {}
}

#[tokio::test]
async fn test_feign() {
    let r = Feign.user("dxx").await.unwrap();
    println!("{}", r);
}

#[derive(Feign)]
struct FeignClient {
    #[param]
    accept: &'static str,
    #[header("content-type")]
    c_type: &'static str,
    #[query(name = "say")]
    say: &'static str,
}
#[feign(url = "http://localhost:1236", headers = "accept: {accept}")]
impl FeignClient {
    #[get]
    async fn home(&self) -> feignhttp::Result<String> {}

    #[get("/repos", headers = "accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String> {}
}

#[tokio::test]
async fn test_feign_client() {
    let _lock = PORT_LOCK.lock().unwrap();
    let opts = ServerOpts {
        port: 1236,
        ..Default::default()
    };
    let mut server = Server::new_with_opts_async(opts).await;
    let _mock_home = server
        .mock("GET", "/")
        .match_header("accept", "application/octet-stream")
        .match_header("content-type", "none")
        .match_query(Matcher::Regex("say=hello".into()))
        .create_async()
        .await;

    let _mock_repo = server
        .mock("GET", "/repos")
        .match_header("accept", "application/json")
        .match_header("content-type", "none")
        .match_query(Matcher::Regex("say=hello".into()))
        .create_async()
        .await;

    let client = FeignClient {
        accept: "application/octet-stream",
        c_type: "none",
        say: "hello",
    };

    client.home().await.unwrap();
    client.repository().await.unwrap();
}
