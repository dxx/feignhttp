use feignhttp::get;

#[get("https://api.github.com")]
async fn github() -> Result<String, Box<dyn std::error::Error>> {}

// Specifies path parameter
#[get("https://api.github.com/repos/{owner}/{repo}")]
async fn repository(
    #[path] owner: &str,
    #[path] repo: String,
) -> Result<String, Box<dyn std::error::Error>> {}

// Specifies path parameter name and query parameter
#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path("owner")] user: &str,
    #[path] repo: &str,
    #[param] page: u32,
) -> Result<String, Box<dyn std::error::Error>> {}


// Specifies header
#[get("https://api.github.com/repos/{owner}/{repo}/commits")]
async fn commits(
    #[header] accept: &str,
    #[path] owner: &str,
    #[path] repo: &str,
    #[param] page: u32,
    #[param] per_page: u32,
) -> Result<String, Box<dyn std::error::Error>> {}


const GITHUB_URL: &str = "https://api.github.com";

// Using url constants
#[get(GITHUB_URL, path = "/repos/{owner}/{repo}/languages")]
async fn languages(
    #[path] owner: &str,
    #[path] repo: &str,
) -> Result<String, Box<dyn std::error::Error>> {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = github().await?;

    println!("github result: {}", r);


    let r = repository("code-mcx", "feignhttp".to_string()).await?;

    println!("repository result: {}", r);


    let r = contributors (
        "code-mcx",
        "feignhttp",
        1
    ).await?;

    println!("contributors result: {}", r);


    let r = commits(
        "application/vnd.github.v3+json",
        "code-mcx",
        "feignhttp",
        1,
        5,
    )
    .await?;

    println!("commits result: {}", r);

    let r = languages("code-mcx", "feignhttp").await?;

    println!("languages result: {}", r);


    Ok(())
}