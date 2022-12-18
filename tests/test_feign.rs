use feignhttp::{feign, get, Feign};

#[get("https://api.github.com")]
pub async fn get() -> feignhttp::Result<String> {}

#[get("https://api.github.com", path = "/abc")]
pub async fn get_not_found() -> feignhttp::Result<String> {}

const URL: &str = "https://api.github.com";

#[derive(Feign)]
pub struct Feign;

#[feign(url = URL)]
impl Feign {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String> {}
}

#[tokio::test]
async fn test_fn() {
    let r = get().await.unwrap();
    println!("{}", r);
}

#[tokio::test]
async fn test_struct() {
    let r = Feign.user("dxx").await.unwrap();
    println!("{}", r);
}

#[tokio::test]
#[should_panic]
async fn test_not_found() {
    // 404
    let _r = get_not_found().await.unwrap();
}
