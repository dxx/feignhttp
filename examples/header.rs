use feignhttp::get;
use serde::Serialize;

// Using `#[header]` to specify header.
#[get("https://api.github.com/repos/dxx/feignhttp/commits")]
async fn commits(
    #[header] accept: &str,
    #[query] page: u32,
    #[query] per_page: u32,
) -> feignhttp::Result<String> {}

// headers format: header-key1: header-value1; header-key2: header-value2; ...
#[get("https://httpbin.org/headers", headers = "key1: value1; key2: value2")]
async fn headers() -> feignhttp::Result<String> {}

#[derive(Serialize)]
pub struct Header {
    pub accept: &'static str,
    pub token: String,
}

#[get("https://httpbin.org/headers")]
async fn struct_headers(#[header] head: Header) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = commits("application/vnd.github.v3+json", 1, 5).await?;
    println!("commits result: {}\n", r);

    let r = headers().await?;
    println!("headers result: {}", r);

    let header = Header {
        accept: "content/json",
        token: String::from("MDEwOlJlcG9zaXRvcnkzNDUyNTk4OTA="),
    };

    let r = struct_headers(header).await?;
    println!("struct headers result: {}", r);

    Ok(())
}
