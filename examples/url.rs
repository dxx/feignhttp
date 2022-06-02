use feignhttp::get;
use once_cell::sync::Lazy;

const GITHUB_URL: &str = "https://api.github.com";

// Using url constants.
#[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<String> {}

// Lazy loading url.
static URL: Lazy<String> = Lazy::new(||
    std::env::var("GITHUB_URL").unwrap_or("https://api.github.com".to_string()));

#[get(url = URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages2(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = languages("dxx", "feignhttp").await?;
    println!("languages result: {}", r);

    let r = languages2("dxx", "feignhttp").await?;
    println!("languages2 result: {}", r);

    Ok(())
}
