use crate::FeignContext;
use crate::{ClientConfig, ClientWrapper, RequestConfig, RequestWrapper, error::Result};
use async_trait::async_trait;
use http::StatusCode;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Define a trait for code generation.
pub trait FeignClientBuilder: Sized {
    type Target;

    fn new() -> Self;

    fn config(self, config: ClientConfig) -> Self;

    fn context<C>(self, context: C) -> Self
    where
        C: FeignContext + 'static;

    fn build(self) -> Result<Self::Target>;
}

/// A trait for HTTP client.
pub trait Client: Sized + Clone {
    type Inner;

    fn new() -> Result<Self>;

    fn with_config(config: ClientConfig) -> Result<Self>;

    fn with_client(client: Self::Inner) -> Result<Self>;

    fn get_client(&self) -> &Self::Inner;
}

/// A trait of HTTP request.
pub trait HttpRequest {
    fn headers(self, headers: HashMap<&str, String>) -> Self;

    fn query(self, query: Vec<(&str, String)>) -> Self;
}

/// A trait of HTTP response.
#[async_trait]
pub trait HttpResponse {
    fn status(&self) -> StatusCode;

    async fn none(self) -> Result<()>;

    async fn text(self) -> Result<String>;

    async fn vec(self) -> Result<Vec<u8>>;

    #[cfg(feature = "json")]
    async fn json<T>(mut self) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
}

/// An HTTP client to create ClientWrapper.
pub struct HttpClient;

impl HttpClient {
    pub fn new() -> Result<ClientWrapper> {
        ClientWrapper::with_config(ClientConfig {
            connect_timeout: Some(10000),
            timeout: Some(10000),
            read_timeout: None,
        })
    }

    pub fn with_config(config: ClientConfig) -> Result<ClientWrapper> {
        ClientWrapper::with_config(config)
    }

    pub fn shared() -> &'static ClientWrapper {
        static SHARED: Lazy<ClientWrapper> =
            Lazy::new(|| HttpClient::new().expect("shared http client failed to initialize"));

        &SHARED
    }
}

/// An HTTP requet builder to make requests.
pub struct RequestBuilder<'a> {
    client: ClientWrapper,
    url: &'a str,
    method: &'a str,
    headers: Option<HashMap<&'a str, String>>,
    query: Option<Vec<(&'a str, String)>>,
    config: Option<RequestConfig>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(client: ClientWrapper) -> Self {
        Self {
            client,
            url: "",
            method: "",
            headers: None,
            query: None,
            config: None,
        }
    }
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = url;
        self
    }

    pub fn method(mut self, method: &'a str) -> Self {
        self.method = method;
        self
    }

    pub fn config(mut self, config: RequestConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn headers(mut self, headers: HashMap<&'a str, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn query(mut self, query: Vec<(&'a str, String)>) -> Self {
        self.query = Some(query);
        self
    }

    pub fn build(self) -> Result<RequestWrapper> {
        let mut request = match self.config {
            Some(config) => {
                RequestWrapper::with_config(self.client, self.url, self.method, config)?
            }
            None => RequestWrapper::new(self.client, self.url, self.method)?,
        };
        if let Some(header_map) = self.headers {
            request = request.headers(header_map);
        }
        if let Some(query_vec) = self.query {
            request = request.query(query_vec);
        }
        Ok(request)
    }
}
