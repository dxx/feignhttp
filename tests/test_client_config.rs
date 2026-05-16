use feignhttp::{ClientConfig, HttpClient, RequestBuilder, RequestConfig};

#[tokio::test]
#[should_panic]
async fn test_default_timeout() {
    let url = "https://httpbin.org/delay/15".to_string();
    let method = "GET";
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_connect_timeout() {
    let url = "http://site_dne.com";
    let method = "GET";
    let mut config = ClientConfig::default();
    config.connect_timeout = Some(3000); // 3000 millisecond.

    let request = RequestBuilder::new(HttpClient::with_config(config).unwrap())
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_timeout() {
    let url = "https://httpbin.org/delay/5".to_string();
    let method = "GET";
    let config = RequestConfig {
        timeout: Some(3000), // 3000 millisecond.
    };
    let request = RequestBuilder::new(HttpClient::new().unwrap())
        .url(&url)
        .method(method)
        .config(config)
        .build()
        .unwrap();
    request.send().await.unwrap();
}
