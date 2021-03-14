#[allow(dead_code)]
mod support;

use feignhttp::{get, feign};

use support::*;

#[get(url = "http://xxx.com", connect_timeout = 3000)]
async fn connect_timeout() -> Result<String, Box<dyn std::error::Error>> {}

#[get(url = "http://localhost:8080", timeout = 3000)]
async fn timeout() -> Result<String, Box<dyn std::error::Error>> {}


pub struct Http {}

#[feign(url = "http://xxx.com", connect_timeout = 3000)]
impl Http {
    #[get("", connect_timeout = 5000)] // 5000 will override 3000
    async fn get() -> Result<String, Box<dyn std::error::Error>> {}
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

    let _server = server::http(8080, move |req| async move {
        assert_eq!(req.method(), "GET");

        std::thread::sleep(std::time::Duration::from_millis(5000));

        hyper::Response::default()
    });

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
