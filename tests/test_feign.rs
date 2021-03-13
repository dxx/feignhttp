use feignhttp::{feign, get};

#[get("https://api.github.com")]
pub async fn get() -> Result<String, Box<dyn std::error::Error>> {}

#[get("https://api.github.com", path = "/abc")]
pub async fn get_not_found() -> Result<String, Box<dyn std::error::Error>> {}


const URL: &str = "https://api.github.com";

pub struct Feign {}

#[feign(url = URL)]
impl Feign {
    #[get("/users/{user}")]
    async fn user(#[path] user: &str) -> Result<String, Box<dyn std::error::Error>> {}
}

#[tokio::test]
async fn test_fn() {
    let r = get().await.unwrap();
    println!("{}", r);
}

#[tokio::test]
async fn test_struct() {
    let r = Feign::user("dxx").await.unwrap();
    println!("{}", r);
}

#[tokio::test]
async fn test_not_found() {
    // 404
    let _r = get_not_found().await.unwrap();
}
