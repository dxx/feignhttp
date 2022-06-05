use std::{io::{Write, Read}, path::PathBuf};

use feignhttp::{get, post};

#[get("https://www.rust-lang.org/static/images/rust-logo-blk.svg")]
async fn get_image() -> feignhttp::Result<Vec<u8>> {}

#[post("https://httpbin.org/anything")]
async fn post_image(#[body] data: Vec<u8>) -> feignhttp::Result<String> {}


fn cargo_dir() -> std::result::Result<String, std::env::VarError> {
    std::env::var("CARGO_MANIFEST_DIR")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = cargo_dir()?;

    let image_data = get_image().await?;
    println!("get image ok: len {}", image_data.len());

    let path = PathBuf::new().join(dir.clone()).join("./examples/rust-logo.svg");
    let mut file = std::fs::File::create(path)?;
    file.write_all(&image_data)?;


    let path = PathBuf::new().join(dir).join("./examples/crab.png");
    let mut file = std::fs::File::open(path)?;
    let mut vec = vec![];
    file.read_to_end(&mut vec)?;

    let r = post_image(vec).await?;
    println!("post image result: {}", r);

    Ok(())
}
