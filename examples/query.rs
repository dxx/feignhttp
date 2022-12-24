use feignhttp::get;

// Using `#[query]` to specify query parameter.
#[get("https://api.github.com/repos/{owner}/{repo}/contributors")]
async fn contributors(
    #[path] owner: &str,
    #[path] repo: &str,
    #[query] page: u32, // `#[query]` can also be removed.
) -> feignhttp::Result<String> {
}

#[get("https://httpbin.org/anything")]
async fn anything(#[query] id: &[i32], #[query] name: Vec<&str>) -> feignhttp::Result<String> {}

#[get("https://httpbin.org/anything")]
async fn anything_vec(
    #[query] id: &[i32],
    #[query] name: &Vec<String>,
) -> feignhttp::Result<String> {
}

use feignhttp::{feign, Feign};

#[derive(Feign)]
struct NameQuery<'a> {
    #[query]
    name: Vec<&'a str>,
}

#[feign(url = "https://httpbin.org/anything")]
impl<'a> NameQuery<'_> {
    #[get]
    async fn anything_name(&self) -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = contributors("dxx", "feignhttp", 1).await?;
    println!("contributors result: {}", r);

    let r = anything(&[1, 2, 3], vec!["Bob", "Tom", "Jack"]).await?;
    println!("anything result: {}", r);

    let names = vec!["Bob".to_string(), "Tom".to_string(), "Jack".to_string()];
    let r = anything_vec(&[1, 2, 3], &names).await?;
    println!("anything vec result: {}", r);

    let t = NameQuery {
        name: vec!["Bob", "Tom", "Jack"],
    };
    let r= t.anything_name().await?;
    println!("anything name result: {}", r);

    Ok(())
}
