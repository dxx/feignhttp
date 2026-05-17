use feignhttp::{FeignClientBuilder, feign, get};

/// The default connect_timeout is 10000 milliseconds.
#[get(url = "http://site_dne.com")]
async fn default_connect_timeout() -> feignhttp::Result<String> {}

/// The default timeout is 10000 milliseconds.
#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn default_timeout() -> feignhttp::Result<String> {}

#[feign(url = "http://site_dne.com")]
pub trait Http {
    #[get("")] // The default connect_timeout and timeout are 10000 milliseconds..
    async fn default_timeout(&self) -> feignhttp::Result<String>;
}

#[tokio::main]
async fn main() {
    match default_connect_timeout().await {
        Ok(res) => {
            println!("default_connect_timeout: {}", res);
        }
        Err(err) => {
            println!("default_connect_timeout: {:?}", err);
        }
    }

    match default_timeout().await {
        Ok(res) => {
            println!("default_timeout: {}", res);
        }
        Err(err) => {
            println!("default_timeout: {:?}", err);
        }
    }

    match Http::builder().build().unwrap().default_timeout().await {
        Ok(res) => {
            println!("Http::default_timeout: {}", res);
        }
        Err(err) => {
            println!("Http::default_timeout: {:?}", err);
        }
    }
}
