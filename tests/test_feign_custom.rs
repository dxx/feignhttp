use feignhttp::{Client, ClientWrapper, FeignClientBuilder, feign};

#[feign(url = "https://api.github.com", headers = "user-agent: Feign HTTP")]
pub trait Feign {
    #[get("/users/{user}")]
    async fn user(&self, #[path] user: &str) -> feignhttp::Result<String>;
}

#[tokio::test]
async fn test_feign() {
    let client_wrapper;

    #[cfg(feature = "reqwest-client")]
    {
        let client = reqwest::Client::new();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }
    #[cfg(feature = "reqwest-middleware-client")]
    {
        use reqwest_middleware::{ClientBuilder, reqwest};

        let reqwest_client = reqwest::Client::new();
        let client = ClientBuilder::new(reqwest_client).build();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }
    #[cfg(feature = "isahc-client")]
    {
        let client = isahc::HttpClient::new().unwrap();
        client_wrapper = ClientWrapper::with_client(client).unwrap();
    }

    let feign = Feign::builder().client(client_wrapper).build().unwrap();
    let r = feign.user("dxx").await.unwrap();

    println!("{}", r);
}
