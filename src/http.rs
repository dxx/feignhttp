use crate::{map, HttpClient};
use reqwest::{Body, Client, Method, RequestBuilder, Response, Url};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

pub struct HttpConfig {
    pub connect_timeout: Option<u16>,
    pub timeout: Option<u16>,
}

impl HttpClient {
    pub fn default_request(url: &str, method: &str) -> RequestWrapper {
        RequestWrapper::build_default(url, method)
    }
    pub fn configure_request(url: &str, method: &str, config: HttpConfig) -> RequestWrapper {
        RequestWrapper::build_with_config(url, method, config)
    }
}

pub struct RequestWrapper {
    headers: HashMap<String, String>,
    request: RequestBuilder,
}

pub struct ResponseWrapper {
    response: Response,
}

impl RequestWrapper {
    pub fn build_default(url: &str, method: &str) -> RequestWrapper {
        let request = Client::new().request(
            Method::from_str(method.to_uppercase().as_str()).unwrap(),
            Url::from_str(url).unwrap(),
        );
        RequestWrapper {
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    pub fn build_with_config(url: &str, method: &str, config: HttpConfig) -> RequestWrapper {
        let mut client = Client::builder();
        if let Some(millisecond) = config.connect_timeout {
            client = client.connect_timeout(Duration::from_millis(millisecond as u64));
        }
        if let Some(millisecond) = config.timeout {
            client = client.timeout(Duration::from_millis(millisecond as u64));
        }
        let request = client.build().unwrap().request(
            Method::from_str(method.to_uppercase().as_str()).unwrap(),
            Url::from_str(url).unwrap(),
        );
        RequestWrapper {
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    pub fn headers(mut self, header_map: HashMap<&str, String>) -> Self {
        for (k, v) in header_map {
            self.headers.insert(k.to_string().to_lowercase(), v);
        }
        self
    }

    fn set_header(mut self) -> Self {
        let mut request = self.request;
        for (k, v) in &self.headers {
            request = request.header(k.as_str(), v);
        }
        self.request = request;
        self
    }

    pub fn query(mut self, query: &Vec<(&str, String)>) -> Self {
        let mut request = self.request;
        request = request.query(query.as_slice());
        self.request = request;
        self
    }

    pub async fn send(self) -> Result<ResponseWrapper, Box<dyn std::error::Error>> {
        let mut response = self.set_header().request.send().await?;
        // Client or server error
        response = response.error_for_status()?;
        Ok(ResponseWrapper { response })
    }

    pub async fn send_text(
        mut self,
        text: String,
    ) -> Result<ResponseWrapper, Box<dyn std::error::Error>> {
        self.request = self.request.body(Body::from(text));
        self.send().await
    }

    pub async fn send_json<T>(
        mut self,
        json: &T,
    ) -> Result<ResponseWrapper, Box<dyn std::error::Error>>
    where
        T: serde::ser::Serialize,
    {
        self.request = self.request.json(json);
        self.send().await
    }
}

impl ResponseWrapper {
    pub fn status(self) -> http::StatusCode {
        self.response.status()
    }
    pub async fn text(self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.response.text().await?)
    }

    pub async fn json<T>(self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self.response.json::<T>().await?)
    }
}
