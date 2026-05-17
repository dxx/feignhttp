use feignhttp::{Client, ClientWrapper, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com", headers = "user-agent: Feign HTTP")]
pub trait GitHub {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String>;

    #[get("/repos/dxx/feignhttp/commits")]
    async fn commits(
        &self,
        #[header] accept: &str,
        #[query] page: u32,
        #[query] per_page: u32,
    ) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client_wrapper;

    #[cfg(feature = "reqwest-client")]
    {
        // You can use reqwest's custom configuration.
        let client = reqwest::Client::builder().build()?;
        client_wrapper = ClientWrapper::with_client(client)?;
    }
    #[cfg(feature = "reqwest-middleware-client")]
    {
        use reqwest_middleware::{ClientBuilder, reqwest};

        let reqwest_client = reqwest::Client::builder().build()?;
        let client = ClientBuilder::new(reqwest_client).build();
        client_wrapper = ClientWrapper::with_client(client)?;
    }
    #[cfg(feature = "isahc-client")]
    {
        let client = isahc::HttpClient::builder().build()?;
        client_wrapper = ClientWrapper::with_client(client)?;
    }

    let github = GitHubBuilder::build_with_client(client_wrapper)?;

    let r = github.user("dxx").await?;
    println!("{}", r);

    let r = github
        .commits("application/vnd.github.v3+json", 1, 1)
        .await?;
    println!("commits result: {}\n", r);

    Ok(())
}
