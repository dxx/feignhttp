use feignhttp::get;

#[get("https://api.github.com")]
pub async fn get() -> feignhttp::Result<String> {}

#[get("https://api.github.com", path = "/abc")]
pub async fn get_not_found() -> feignhttp::Result<String> {}

#[tokio::test]
async fn test_fn() {
    let r = get().await.unwrap();
    println!("{}", r);
}

#[tokio::test]
#[should_panic]
async fn test_not_found() {
    // 404
    let _r = get_not_found().await.unwrap();
}
