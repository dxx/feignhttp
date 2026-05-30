#![allow(unused_imports)]

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
        use reqwest_middleware::{ClientBuilder, Middleware, Next, reqwest};
        use http::Extensions;

        struct LoggingMiddleware;

        #[async_trait::async_trait]
        impl Middleware for LoggingMiddleware {
            async fn handle(
                &self,
                req: reqwest::Request,
                extensions: &mut Extensions,
                next: Next<'_>,
            ) -> reqwest_middleware::Result<reqwest::Response> {
                println!("Request started {:?}\n", req);
                let res = next.run(req, extensions).await;
                println!("Result: {:?}\n", res);
                res
            }
        }

        let reqwest_client = reqwest::Client::builder().build()?;

        // See https://github.com/TrueLayer/reqwest-middleware.
        let client = ClientBuilder::new(reqwest_client)
            .with(LoggingMiddleware)
            .build();

        // Create Feign ClientWrapper.
        let client_wrapper = ClientWrapper::with_client(client)?;

        // Build client with custom client.
        let feign = FeignBuilder::build_with_client(client_wrapper).unwrap();
        let _r = feign.user("dxx").await.unwrap();
    }

    Ok(())
}
