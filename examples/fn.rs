use feignhttp::get;

#[get("https://api.github.com")]
async fn github() -> feignhttp::Result<String> {}

// Specifies path parameter
#[get("https://api.github.com/repos/{owner}/{repo}")]
async fn repository(
    #[path] owner: &str,
    #[path] repo: String,
) -> feignhttp::Result<String> {}

// Specifies path parameter name and query parameter
#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path("owner")] user: &str,
    #[path] repo: &str,
    #[query] page: u32,
) -> feignhttp::Result<String> {}


// Specifies header
#[get("https://api.github.com/repos/{owner}/{repo}/commits")]
async fn commits(
    #[header] accept: &str,
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32,
    #[query] per_page: u32,
) -> feignhttp::Result<String> {}


const GITHUB_URL: &str = "https://api.github.com";

// Using url constants
#[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages(
    #[path] owner: &str,
    #[path] repo: &str,
) -> feignhttp::Result<String> {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "feignhttp=debug");
    env_logger::init();

    let r = github().await?;

    println!("github result: {}", r);


    let r = repository("dxx", "feignhttp".to_string()).await?;

    println!("repository result: {}", r);


    let r = contributors (
        "dxx",
        "feignhttp",
        1
    ).await?;

    println!("contributors result: {}", r);


    let r = commits(
        "application/vnd.github.v3+json",
        "dxx",
        "feignhttp",
        1,
        5,
    )
    .await?;

    println!("commits result: {}", r);

    let r = languages("dxx", "feignhttp").await?;

    println!("languages result: {}", r);


    Ok(())
}