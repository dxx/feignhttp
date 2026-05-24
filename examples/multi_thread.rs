use feignhttp::{FeignClientBuilder, feign};
use std::sync::Arc;

#[feign(
    url = "https://httpbin.org/headers",
    headers = "Authorization: Bearer {token}"
)]
pub trait UserClient {
    #[get]
    async fn get_headers(&self, #[param] token: &str) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = vec!["token_a", "token_b", "token_c"];

    let client = Arc::new(UserClient::builder().build()?);

    let handles: Vec<_> = tokens
        .into_iter()
        .map(|token| {
            let client = client.clone();
            async move {
                let r = client.get_headers(token).await;
                println!("token: {}, result: {}", token, r.unwrap());
            }
        })
        .map(|fut| tokio::spawn(fut))
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
