use feignhttp::get;

// Using `#[header]` to specify header.
#[get("https://api.github.com/repos/{owner}/{repo}/commits")]
async fn commits(
    #[header] accept: &str,
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32,
    #[query] per_page: u32,
) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = commits(
        "application/vnd.github.v3+json",
        "dxx",
        "feignhttp",
        1,
        5,
    )
    .await?;
    println!("commits result: {}", r);

    Ok(())
}
