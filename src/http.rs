use crate::{error::Result, RequestWrapper};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct HttpClient;

impl HttpClient {
    pub fn default_request(url: &str, method: &str) -> RequestWrapper {
        RequestWrapper::build_default(url, method)
    }
    pub fn configure_request(url: &str, method: &str, config: HttpConfig) -> RequestWrapper {
        RequestWrapper::build_with_config(url, method, config)
    }
}

pub struct HttpConfig {
    pub connect_timeout: Option<u16>,
    pub timeout: Option<u16>,
}

impl HttpConfig {
    pub fn from_map(config_map: HashMap<&str, String>) -> Self {
        let mut config = HttpConfig {
            connect_timeout: None,
            timeout: None,
        };
        if let Some(connect_timeout) = config_map.get("connect_timeout") {
            config.connect_timeout = Some(connect_timeout.parse::<u16>().unwrap());
        }
        if let Some(timeout) = config_map.get("timeout") {
            config.timeout = Some(timeout.parse::<u16>().unwrap());
        }
        config
    }
}

pub trait HttpRequest {
    fn build_default(url: &str, method: &str) -> Self;

    fn build_with_config(url: &str, method: &str, config: HttpConfig) -> Self;

    fn headers(self, header_map: HashMap<&str, String>) -> Self;

    fn query(self, query: &Vec<(&str, String)>) -> Self;
}

#[async_trait]
pub trait HttpResponse {
    fn status(self) -> http::StatusCode;

    async fn text(self) -> Result<String>;
}
