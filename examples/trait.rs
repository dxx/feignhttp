use feignhttp::{Context, FeignClientBuilder, feign};

#[derive(Context)]
struct GithubContext {
    // `url_path` and `param` are used to set the shared data.
    // The other two for share data are `header` and `query`.
    #[url_path("owner")]
    user: &'static str,
    #[url_path]
    repo: &'static str,
    #[param]
    accept: &'static str,
}

/// Generate Github and GithubBuilder structures and some methods.
#[feign(
    url = "https://api.github.com/repos/{owner}/{repo}",
    headers = "Accept: {accept}"
)]
pub trait Github {
    // The method must have a self argument.
    #[get]
    async fn home(&self) -> feignhttp::Result<String>;

    #[get(path = "", headers = "Accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String>;

    #[get(path = "/contributors")]
    async fn contributors(&self, #[query] page: u32) -> feignhttp::Result<String>;

    #[get("/commits")]
    async fn commits(
        &self,
        #[header] accept: &str,
        #[query] page: u32,
        #[query] per_page: u32,
    ) -> feignhttp::Result<String>;

    #[get(path = "/languages")]
    async fn languages(&self) -> feignhttp::Result<String>;  
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = GithubContext {
        user: "dxx",
        repo: "feignhttp",
        accept: "*/*",
    };

    // The builder method generate a `GithubBuilder` instance
    // and the build method generate a `Github` instance.
    let dxx_github = Github::builder().context(context).build()?;

    let r = dxx_github.home().await?;
    println!("github result: {}\n", r);

    let r = dxx_github.repository().await?;
    println!("repository result: {}\n", r);

    let r = dxx_github.contributors(1).await?;
    println!("contributors result: {}\n", r);

    let r = dxx_github
        .commits("application/vnd.github.v3+json", 1, 1)
        .await?;
    println!("commits result: {}\n", r);

    let r = dxx_github.languages().await?;
    println!("languages result: {}\n", r);

    Ok(())
}
