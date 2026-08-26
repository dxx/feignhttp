use feignhttp::{delete, get, patch, post, put};

#[get("https://httpbin.org/get")]
async fn get() -> feignhttp::Result<String> {}

#[post("https://httpbin.org/post")]
async fn post(#[body] data: &str) -> feignhttp::Result<String> {}

#[put("https://httpbin.org/put")]
async fn put(#[body] data: &str) -> feignhttp::Result<String> {}

#[delete("https://httpbin.org/delete")]
async fn delete() -> feignhttp::Result<String> {}

#[patch("https://httpbin.org/patch")]
async fn patch(#[body] data: &str) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = get().await?;
    println!("get result: {}", r);

    let r = post("hello").await?;
    println!("post result: {}", r);

    let r = put("hello").await?;
    println!("put result: {}", r);

    let r = delete().await?;
    println!("delete result: {}", r);

    let r = patch("hello").await?;
    println!("patch result: {}", r);

    Ok(())
}
