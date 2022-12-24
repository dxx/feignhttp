use feignhttp::{feign, Feign};

use mockito::{mock, Matcher};

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
    #[query(name="say")]
    say: &'static str,
}
#[feign(url = "http://localhost:1234", headers = "accept: {accept}")]
impl FeignClient {
    #[get]
    async fn home(&self) -> feignhttp::Result<String> {}

    #[get("/repos", headers = "accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String> {}
}

#[tokio::test]
async fn test_feign_client() {
    let _mock_home = mock("GET", "/")
        .match_header("accept", "application/octet-stream")
        .match_header("content-type", "none")
        .match_query(Matcher::Regex("say=hello".into()))
        .create();

    let _mock_repo = mock("GET", "/repos")
        .match_header("accept", "application/json")
        .match_header("content-type", "none")
        .match_query(Matcher::Regex("say=hello".into()))
        .create();

    let client = FeignClient {
        accept: "application/octet-stream",
        c_type: "none",
        say: "hello"
    };

    client.home().await.unwrap();
    client.repository().await.unwrap();
}
