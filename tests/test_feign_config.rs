use feignhttp::get;

#[get(url = "http://site_dne.com", connect_timeout = 3000)]
async fn connect_timeout() -> feignhttp::Result<String> {}

#[tokio::test]
#[should_panic]
async fn test_connect_timeout() {
    connect_timeout().await.unwrap();
}

#[get(url = "https://httpbin.org/delay/5", timeout = 3000)]
async fn timeout() -> feignhttp::Result<String> {}

#[tokio::test]
#[should_panic]
async fn test_timeout() {
    timeout().await.unwrap();
}

#[get(url = "https://httpbin.org/delay/3", timeout = "{time}")]
async fn dynamic_timeout(#[param] time: u16) -> feignhttp::Result<String> {}

#[tokio::test]
#[should_panic]
async fn test_dynamic_timeout1() {
    dynamic_timeout(2000).await.unwrap();
}

#[tokio::test]
async fn test_dynamic_timeout2() {
    dynamic_timeout(5000).await.unwrap();
}
