#[allow(dead_code)]
mod support;

use feignhttp::{get, post};
use serde::{Deserialize, Serialize};

use support::*;
use hyper::body::HttpBody;

// Deserialize: Specifies deserialization
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
) -> Result<Vec<IssueItem>, Box<dyn std::error::Error>> {}


#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post(url = "http://localhost:8080/create")]
async fn create_user(#[body] user: User) -> Result<String, Box<dyn std::error::Error>> {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = issues("octocat", "hello-world", 1, 2).await?;
    println!("issues: {:#?}", r);


    let _server = server::http(8080, move |mut req| async move {
        let vec = req.body_mut().data().await.unwrap().unwrap().to_vec();
        println!("method: {}", req.method());
        println!("received: {}", String::from_utf8(vec).unwrap());

        hyper::Response::default()
    });

    let user = User {
        id: 1,
        name: "jack".to_string(),
    };
    let _r = create_user(user).await?;


    Ok(())
}
