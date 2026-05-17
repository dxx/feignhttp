use feignhttp::{Context, FeignClientBuilder as _, feign};

use mockito::{Matcher, Server, ServerOpts};

const URL: &str = "https://api.github.com";

#[feign(url = URL)]
pub trait Feign {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String>;
}

#[tokio::test]
async fn test_feign() {
    let r = Feign::builder().build().unwrap().user("dxx").await.unwrap();

    println!("{}", r);
}

#[derive(Context)]
struct FeignContext {
    #[param]
    accept: &'static str,
    #[header("content-type")]
    c_type: &'static str,
    #[query(name = "say")]
    say: &'static str,
}

#[feign(url = "http://localhost:1237", headers = "accept: {accept}")]
pub trait FeignClient {
    #[get]
    async fn home(&self) -> feignhttp::Result<String>;

    #[get("/repos", headers = "accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String>;
}

#[tokio::test]
async fn test_feign_client() {
    let opts = ServerOpts {
        port: 1237,
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

    let context = FeignContext {
        accept: "application/octet-stream",
        c_type: "none",
        say: "hello",
    };

    let client = FeignClient::builder().context(context).build().unwrap();

    client.home().await.unwrap();
    client.repository().await.unwrap();
}
