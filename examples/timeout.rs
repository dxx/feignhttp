use feignhttp::{FeignClientBuilder, feign, get};

/// The default connect_timeout is 10000 milliseconds.
#[get(url = "http://site_dne.com")]
async fn default_connect_timeout() -> feignhttp::Result<String> {}

/// The default timeout is 10000 milliseconds, override the default timeout.
#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn default_timeout() -> feignhttp::Result<String> {}

/// The default timeout for all requests in the trait is 10000 milliseconds.
#[feign(url = "https://httpbin.org")]
pub trait TimeoutClient {
    // The default timeout is 10000 milliseconds.
    #[get(path = "/delay/5")]
    async fn default_timeout(&self) -> feignhttp::Result<String>;

    // Override the default timeout.
    #[get(path = "/delay/5", timeout = 3000)]
    async fn custom_timeout(&self) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match default_connect_timeout().await {
        Ok(res) => {
            println!("default_connect_timeout ok: {}", res);
        }
        Err(err) => {
            // Execute here.
            println!("default_connect_timeout err: {:?}", err);
        }
    }

    match default_timeout().await {
        Ok(res) => {
            println!("default_timeout ok: {}", res);
        }
        Err(err) => {
            // Execute here.
            println!("default_timeout err: {:?}", err);
        }
    }

    let http_client = TimeoutClient::builder().build()?;
    match http_client.default_timeout().await {
        Ok(res) => {
            // Execute here.
            println!("TimeoutClient.default_timeout ok: {}", res);
        }
        Err(err) => {
            println!("TimeoutClient.default_timeout err: {:?}", err);
        }
    }

    match http_client.custom_timeout().await {
        Ok(res) => {
            println!("TimeoutClient.custom_timeout ok: {}", res);
        }
        Err(err) => {
            // Execute here.
            println!("TimeoutClient.custom_timeout err: {:?}", err);
        }
    }

    Ok(())
}
