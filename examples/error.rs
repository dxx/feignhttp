use feignhttp::{get, ErrorKind};

#[get("httpbin.org/anything")]
async fn url_error() -> feignhttp::Result<()> {}

#[get(url = "https://httpbin.org/delay/3", timeout = "abc")]
async fn config_error() -> feignhttp::Result<()> {}

#[get(url = "https://httpbin.org/delay/3", timeout = 2000)]
async fn timeout_error() -> feignhttp::Result<()> {}

#[get(url = "https://httpbin.org/123")]
async fn status_error() -> feignhttp::Result<()> {}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct User {
    id: i32,
}

#[cfg(feature = "json")]
#[get(url = "https://httpbin.org/anything")]
async fn decode_error() -> feignhttp::Result<User> {}

#[tokio::main]
async fn main() {
    match url_error().await {
        Err(err) => {
            // Build client error.
            if err.is_build_error() {
                println!("url_error: {}", err);
            }
        },
        _ => {}
    }

    match config_error().await {
        Err(err) => {
            // Parse config error.
            if err.is_config_error() {
                println!("config_error: {}", err);
            }
        },
        _ => {}
    }

    match timeout_error().await {
        Err(err) => {
            // Request error.
            if err.is_request_error() {
                println!("timeout_error: {}", err);
            }
        },
        _ => {}
    }

    #[cfg(feature = "json")]
    match decode_error().await {
        Err(err) => {
            // Decode error.
            if err.is_decode_error() {
                println!("decode_error: {}", err);
            }
        },
        _ => {}
    }

    match status_error().await {
        Err(err) => {
            // Status error.
            if err.is_status_error() {
                println!("status_error: {}", err);
            }
            if let ErrorKind::Status(status) = err.error_kind() {
                
                println!("status error code: {}", status.as_u16());

                if status.is_client_error() {
                    // Handle error.
                }
                if status.is_server_error() {
                    // Handle error.
                }
            }
        },
        _ => {}
    }
}
