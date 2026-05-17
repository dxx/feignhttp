
use feignhttp::{Client, ClientWrapper, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com", headers = "user-agent: Feign HTTP")]
pub trait Feign {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[cfg(feature = "reqwest-middleware-client")]
    {
        use reqwest_middleware::{ClientBuilder, reqwest};
        use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
        use reqwest_tracing::TracingMiddleware;

        let reqwest_client = reqwest::Client::builder().build()?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        
        // See https://github.com/TrueLayer/reqwest-middleware.
        let client = ClientBuilder::new(reqwest_client)
                // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(TracingMiddleware::default())
            // Retry failed requests.
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        // Create Feign ClientWrapper.
        let client_wrapper = ClientWrapper::with_client(client)?;

        // Build client with custom client.
        let feign = FeignBuilder::build_with_client(client_wrapper).unwrap();
        let r = feign.user("dxx").await.unwrap();

        println!("{}", r);
    }

    Ok(())
}
