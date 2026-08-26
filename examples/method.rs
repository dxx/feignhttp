use feignhttp::patch;

#[patch("https://httpbin.org/patch")]
async fn patch() -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = patch().await?;
    println!("patch result: {}", r);

    Ok(())
}
