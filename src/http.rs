use crate::{HttpClient, map};
use std::collections::HashMap;
use std::str::FromStr;
use reqwest::{Client, Method, RequestBuilder, Url, Response, Body};

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new()
    }
}

impl HttpClient {
    pub fn build_request(self, url: &str, method: &str) -> RequestWrapper {
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
}

pub struct RequestWrapper {
    headers: HashMap<String, String>,
    request: RequestBuilder,
}

pub struct ResponseWrapper {
    response: Response,
}

impl RequestWrapper {
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

    pub async fn send_text(mut self, text: String) -> Result<ResponseWrapper, Box<dyn std::error::Error>> {
        self.request = self.request.body(Body::from(text));
        self.send().await
    }

    pub async fn send_json<T>(mut self, json: &T) -> Result<ResponseWrapper, Box<dyn std::error::Error>>
    where
        T: serde::ser::Serialize
    {
        self.request = self.request.json(json);
        self.send().await
    }
}

impl ResponseWrapper {
    pub fn status(self) -> http::StatusCode {
        self.response.status()
    }
    pub async fn text(self) -> Result<String, Box<dyn std::error::Error>>  {
        Ok(self.response.text().await?)
    }

    pub async fn json<T>(self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned
    {
        Ok(self.response.json::<T>().await?)
    }
}
