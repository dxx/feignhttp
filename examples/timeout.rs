use feignhttp::{Feign, feign, get};

/// The default connect_timeout is 10000 milliseconds.
#[get(url = "http://site_dne.com")]
async fn connect_timeout() -> feignhttp::Result<String> {}

/// The default timeout is 10000 milliseconds.
#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}

#[derive(Feign)]
struct Http;

#[feign(url = "http://site_dne.com")]
impl Http {
    #[get("")] // The default connect_timeout is 10000 milliseconds..
    async fn get(&self) -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() {
    match connect_timeout().await {
        Ok(res) => {
            println!("connect_timeout: {}", res);
        }
        Err(err) => {
            println!("connect_timeout: {:?}", err);
        }
    }

    match timeout().await {
        Ok(res) => {
            println!("timeout: {}", res);
        }
        Err(err) => {
            println!("timeout: {:?}", err);
        }
    }

    match Http.get().await {
        Ok(res) => {
            println!("Http::get: {}", res);
        }
        Err(err) => {
            println!("Http::get: {:?}", err);
        }
    }
}
