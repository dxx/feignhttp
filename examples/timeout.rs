use feignhttp::{get, feign};

#[get(url = "http://site_dne.com", connect_timeout = 3000)]
async fn connect_timeout() -> feignhttp::Result<String> {}

#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}

pub struct Http;

#[feign(url = "http://site_dne.com", connect_timeout = 3000)]
impl Http {
    #[get("", connect_timeout = 5000)] // 5000 will override 3000.
    async fn get() -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match connect_timeout().await {
        Ok(res) => {
            println!("connect_timeout: {}", res);
        },
        Err(err) => {
            println!("connect_timeout: {:?}", err);
        }
    }

    match timeout().await {
        Ok(res) => {
            println!("timeout: {}", res);
        },
        Err(err) => {
            println!("timeout: {:?}", err);
        }
    }

    match Http::get().await {
        Ok(res) => {
            println!("Http::get: {}", res);
        },
        Err(err) => {
            println!("Http::get: {:?}", err);
        }
    }

    Ok(())
}
