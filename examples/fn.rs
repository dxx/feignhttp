use feignhttp::{get, post};

#[get("https://api.github.com")]
async fn get() -> Result<String, Box<dyn std::error::Error>> {}

#[get("https://api.github.com/users/{user}")]
async fn get_user(#[path] user: &str) -> Result<String, Box<dyn std::error::Error>> {}

#[get("https://api.github.com/users/{username}/followers")]
async fn get_followers(
    #[path("username")] user: String,
    #[param] page: u32,
    #[param] per_page: u32,
) -> Result<String, Box<dyn std::error::Error>> {}

#[tokio::main]
async fn main() {
    let r = get().await.unwrap();
    println!("get result: {}", r);

    let r = get_user("code-mcx").await.unwrap();
    println!("get_user result: {}", r);

    let r = get_followers(
        "code-mcx".to_string(),
        1, 5
    ).await.unwrap();

    println!("get_followers result: {}", r);
}