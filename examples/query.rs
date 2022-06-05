use feignhttp::get;

// Using `#[query]` to specify query parameter.
#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32, // `#[query]` can also be removed.
) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = contributors("dxx", "feignhttp", 1).await?;
    println!("contributors result: {}", r);

    Ok(())
}
