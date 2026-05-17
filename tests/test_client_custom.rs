use feignhttp::RequestBuilder;

use mockito::Server;

#[tokio::test]
async fn test_custom_client() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/").create_async().await;

    let url = format!("http://{}", server.host_with_port());
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
        .url(url.as_str())
        .method(method)
        .build()
        .unwrap();
    request.send().await.unwrap();
}
