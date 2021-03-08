use feignhttp::feign;

const GITHUB_URL: &str = "https://api.github.com";

struct Github {}

#[feign(url = GITHUB_URL)]
impl Github {
    #[get]
    async fn home() -> Result<String, Box<dyn std::error::Error>> {}

    #[get("/repos/{owner}/{repo}")]
    async fn repository(
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {}

    #[get(path = "/repos/{owner}/{repo}/contributors")]
    async fn contributors(
        #[path("owner")] user: &str,
        #[path] repo: &str,
        #[param] page: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {}

    #[get("/repos/{owner}/{repo}/commits")]
    async fn commits(
        #[header] accept: &str,
        #[path] owner: &str,
        #[path] repo: &str,
        #[param] page: u32,
        #[param] per_page: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {}

    // Structure method still send request
    #[get(path = "/repos/{owner}/{repo}/languages")]
    async fn languages(
        &self,
        #[path] owner: &str,
        #[path] repo: &str,
    )-> Result<String, Box<dyn std::error::Error>> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Github::home().await?;

    println!("github result: {}", r);


    let r = Github::repository("code-mcx", "feignhttp").await?;

    println!("repository result: {}", r);


    let r = Github::contributors("code-mcx", "feignhttp", 1).await?;

    println!("contributors result: {}", r);


    let r = Github::commits(
        "application/vnd.github.v3+json",
        "code-mcx",
        "feignhttp",
        1,
        1,
    )
    .await?;

    println!("commits result: {}", r);


    let r = Github {}.languages("code-mcx", "feignhttp").await?;

    println!("languages result: {}", r);


    Ok(())
}
