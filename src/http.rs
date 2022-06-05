use crate::{error::{Result, Error}, RequestWrapper};
use async_trait::async_trait;
use std::collections::HashMap;

/// An HTTP client to create RequestBuilder.
pub struct HttpClient;

impl HttpClient {
    pub fn builder<'a>() -> RequestBuilder<'a> {
        RequestBuilder::new()
    }
}

/// An HTTP requet builder to make requests.
pub struct RequestBuilder<'a> {
    url: &'a str,
    method: &'a str,
    headers: Option<HashMap<&'a str, String>>,
    query: Option<Vec<(&'a str, String)>>,
    config: Option<HttpConfig>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
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

    pub fn config(mut self, config: HttpConfig) -> Self {
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
            Some(config) => RequestWrapper::build_with_config(self.url, self.method, config)?,
            None => RequestWrapper::build_default(self.url, self.method)?,
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

/// Configuration of an HTTP request.
pub struct HttpConfig {
    pub connect_timeout: Option<u64>,
    pub timeout: Option<u64>,
}

impl HttpConfig {
    pub fn from_map(config_map: HashMap<&str, String>) -> Result<Self> {
        let mut config = HttpConfig {
            connect_timeout: None,
            timeout: None,
        };
        if let Some(connect_timeout) = config_map.get("connect_timeout") {
            config.connect_timeout = Some(connect_timeout.parse::<u64>().map_err(Error::config)?);
        }
        if let Some(timeout) = config_map.get("timeout") {
            config.timeout = Some(timeout.parse::<u64>().map_err(Error::config)?);
        }
        Ok(config)
    }
}

/// A trait of HTTP request.
pub trait HttpRequest {
    fn headers(self, headers: HashMap<&str, String>) -> Self;

    fn query(self, query: Vec<(&str, String)>) -> Self;
}

/// A trait of HTTP response.
#[async_trait]
pub trait HttpResponse {
    fn status(&self) -> http::StatusCode;

    async fn none(self) -> Result<()>;

    async fn text(self) -> Result<String>;

    async fn vec(self) -> Result<Vec<u8>>;
}
