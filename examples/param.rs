use feignhttp::{feign, get};

#[get("https://httpbin.org/headers", headers = "token: {token}")]
async fn headers(#[param] token: &str) -> feignhttp::Result<String> {}

#[get(url = "https://httpbin.org/delay/5", timeout = "{time}")]
async fn timeout(#[param] time: u16) -> feignhttp::Result<String> {}

#[get(url = "https://httpbin.org/delay/{delay_time}", timeout = "{time}")]
async fn dynamic_timeout(
    #[path] delay_time: u8, // Replace `{delay_time}` in url.
    #[param] time: u16      // Replace `{time}`.
) -> feignhttp::Result<String> {}

struct Http;

#[feign(url = "https://httpbin.org/delay/5", timeout = "{timeout}")]
impl Http {
    #[get]
    async fn timeout(
        #[param("timeout")] time: u16, // Replace `{timeout}` in feign attribute.
    ) -> feignhttp::Result<String> {}

    #[get(path = "", timeout = "{time}")] // Override timeout in feign attribute.
    async fn override_timeout(
        #[param(name = "time")] time: &str, // Must be a type that can be converted to timeout.
    ) -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() {
    // A reqeut with a header `token: ZmVpZ25odHRw`.
    let res = headers("ZmVpZ25odHRw").await.unwrap();
    println!("headers: {}", res);


    // Request timeout is 3000ms.
    match timeout(3000).await {
        Ok(res) => {
            println!("timeout(3000) ok: {}\n", res);
        }
        Err(err) => {
            // Execute here.
            println!("timeout(3000) err: {:?}\n", err);
        }
    }

    // The request url is https://httpbin.org/delay/5 and the request timeout is 3000ms.
    match dynamic_timeout(5, 3000).await {
        Ok(res) => {
            println!("dynamic_timeout(5, 3000) ok: {}\n", res);
        }
        Err(err) => {
            // Execute here.
            println!("dynamic_timeout(5, 3000) err: {:?}\n", err);
        }
    }

    // The request url is https://httpbin.org/delay/1 and the request timeout is 3000ms.
    match dynamic_timeout(1, 3000).await {
        Ok(res) => {
            // Execute here.
            println!("dynamic_timeout(1, 3000) ok: {}\n", res);
        }
        Err(err) => {
            println!("dynamic_timeout(1, 3000) err: {:?}\n", err);
        }
    }

    // The request timeout is 3000ms.
    match Http::timeout(3000).await {
        Ok(res) => {
            println!("Http::timeout(3000) ok: {}\n", res);
        }
        Err(err) => {
            // Execute here.
            println!("Http::timeout(3000) err: {:?}\n", err);
        }
    }

    // The request timeout is 7000ms.
    match Http::override_timeout("7000").await {
        Ok(res) => {
            // Execute here.
            println!("Http::override_timeout(\"7000\") ok: {}\n", res);
        }
        Err(err) => {
            println!("Http::override_timeout(\"7000\") err: {:?}\n", err);
        }
    }
}
