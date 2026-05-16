use feignhttp::{RequestBuilder};

use mockito::{mock, server_address};

#[tokio::test]
async fn test_custom_client() {
    let _mock = mock("GET", "/").create();

    let url = format!("http://{}", server_address());
    let method = "GET";

    use feignhttp::{Client, ClientWrapper};

    let client_wrapper;

    #[cfg(feature = "reqwest-client")]
    {
        let client = reqwest::Client::new();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }
    #[cfg(feature = "reqwest-middleware-client")]
    {
        use reqwest_middleware::{ClientBuilder, reqwest};
        let client = ClientBuilder::new(reqwest::Client::new()).build();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }
    #[cfg(feature = "isahc-client")]
    {
        let client = isahc::HttpClient::new().unwrap();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }
    
    let request = RequestBuilder::new(client_wrapper)
        .url(&url)
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}
