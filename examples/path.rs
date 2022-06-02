use feignhttp::get;

// Using `#[path]` to specify path parameter.
#[get("https://api.github.com/repos/{owner}/{repo}")]
async fn repository(
    #[path("owner")] user: &str,
    #[path] repo: String,
) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = repository("dxx", "feignhttp".to_string()).await?;
    println!("repository result: {}", r);

    Ok(())
}
