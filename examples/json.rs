#![allow(dead_code)]

use feignhttp::{get, post};

use serde::{Deserialize, Serialize};

// Deserialize: Specifies deserialization.
#[derive(Debug, Deserialize)]
struct IssueItem {
    pub id: u32,
    pub number: u32,
    pub title: String,
    pub url: String,
    pub repository_url: String,
    pub state: String,
    pub body: Option<String>,
}


const GITHUB_URL: &str = "https://api.github.com";

#[get(url = GITHUB_URL, path = "/repos/{owner}/{repo}/issues")]
async fn issues(
    #[path] owner: &str,
    #[path] repo: &str,
    page: u32,
    per_page: u32,
) -> feignhttp::Result<Vec<IssueItem>> {}


#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "https://httpbin.org/anything")]
async fn post_user(#[body] user: User) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "feignhttp=debug");
    env_logger::init();

    let r = issues("octocat", "hello-world", 1, 2).await?;
    println!("issues: {:#?}", r);

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let r = post_user(user).await?;
    println!("{}", r);

    Ok(())
}
