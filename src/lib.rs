pub mod http;
pub mod util;

mod macros;

pub use feignhttp_codegen::*;

pub struct HttpClient {}

//const URL: &str = "http://www.baidu.com";

// #[get(url = URL)]
// pub async fn http_get(#[param] aa: [i32; 5], bb: i32) -> Result<String, String> {}

//struct Http {}

// #[feign("http://www.baidu.com")]
//impl Http {
    // #[get("/11111111")]
    // pub async fn get() -> Result<String, String> {}
//}
