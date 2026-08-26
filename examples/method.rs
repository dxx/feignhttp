use feignhttp::{head, options, patch};

#[patch("https://httpbin.org/patch")]
async fn patch() -> feignhttp::Result<String> {}

#[head("https://httpbin.org/get")]
async fn head() -> feignhttp::Result<()> {}

#[options("https://httpbin.org")]
async fn options() -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = patch().await?;
    println!("patch result: {}", r);

    head().await?;
    println!("head ok");

    let r = options().await?;
    println!("options result: {}", r);

    Ok(())
}
