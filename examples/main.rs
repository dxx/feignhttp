use feign_req::{feign, get};


const URL: &str = "http://www.baidu.com";

#[get("https://www.sina.com.cn/")]
pub async fn http_get() -> Result<String, Box<dyn std::error::Error>> {}

pub struct Http {}

#[feign(url = URL, path="/aaaaaaaa")]
impl Http {
    #[get("/{aaa}/{id}")]
    pub async fn get(#[path("aaa")] aaa: &str, #[path] id: u32) -> Result<String, Box<dyn std::error::Error>> {}
}

#[tokio::main]
async fn main() {
    let _r = http_get().await.unwrap();

    let _r = Http::get("user", 1).await.unwrap();
}
