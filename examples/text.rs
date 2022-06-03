use feignhttp::post;

#[post(url = "https://httpbin.org/anything")]
async fn post_data(#[body] data: String) -> feignhttp::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = String::from("{ \"id\": 1, \"name\": \"jack\" }");
    let r = post_data(data).await?;
    println!("{}", r);

    Ok(())
}
