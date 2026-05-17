use feignhttp::{ClientConfig, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com")]
pub trait GitHub {
    #[get("/repos/{owner}/{repo}")]
    async fn repository(
        &self,
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use ClientConfig to configure timeouts.
    let config = ClientConfig {
        connect_timeout: Some(5000), // 5 seconds
        timeout: Some(10000),        // 10 seconds
        read_timeout: Some(5000),    // 5 seconds
    };

    // Apply config to a trait builder.
    let github = GitHub::builder().config(config).build()?;

    let r = github.repository("dxx", "feignhttp").await?;
    println!("repository: {}\n", r);

    Ok(())
}
