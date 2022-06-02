use crate::{
    error::Error, error::ErrorKind, error::Result, http::HttpConfig, http::HttpRequest,
    http::HttpResponse, map,
};
use super::log::{print_request_log, print_response_log};
use async_trait::async_trait;
use http::StatusCode;
use reqwest::{Body, Client, Method, RequestBuilder, Response};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

/// A wrapper of HTTP request.
pub struct RequestWrapper {
    url: Url,
    headers: HashMap<String, String>,
    request: RequestBuilder,
}

/// A wrapper of HTTP response.
pub struct ResponseWrapper {
    response: Response,
}

impl HttpRequest for RequestWrapper {
    fn build_default(url: &str, method: &str) -> RequestWrapper {
        let url = Url::from_str(url).unwrap();
        let request = Client::new().request(
            Method::from_str(method.to_uppercase().as_str()).unwrap(),
            url.clone(),
        );
        RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    fn build_with_config(url: &str, method: &str, config: HttpConfig) -> RequestWrapper {
        let mut client = Client::builder();
        if let Some(millisecond) = config.connect_timeout {
            client = client.connect_timeout(Duration::from_millis(millisecond as u64));
        }
        if let Some(millisecond) = config.timeout {
            client = client.timeout(Duration::from_millis(millisecond as u64));
        }
        let url = Url::from_str(url).unwrap();
        let request = client.build().unwrap().request(
            Method::from_str(method.to_uppercase().as_str()).unwrap(),
            url.clone(),
        );
        RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    fn headers(mut self, header_map: HashMap<&str, String>) -> Self {
        for (k, v) in header_map {
            self.headers.insert(k.to_string().to_lowercase(), v);
        }
        self
    }

    fn query(mut self, query: &Vec<(&str, String)>) -> Self {
        if query.len() == 0 {
            return self;
        }
        let mut request = self.request;
        request = request.query(query.as_slice());
        if let Some(builder) = request.try_clone() {
            if let Ok(ref req) = builder.build() {
                // Change url after add query parameters.
                self.url = req.url().clone();
            }
        }
        self.request = request;
        self
    }
}

impl RequestWrapper {
    fn set_header(mut self) -> Self {
        let mut request = self.request;
        for (k, v) in &self.headers {
            request = request.header(k.as_str(), v);
        }
        self.request = request;
        self
    }

    fn set_header_if_absent(&mut self, k: &str, v: String) {
        if let None = self.headers.get(k) {
            self.headers.insert(k.to_string(), v);
        }
    }

    async fn send_body(mut self, body: Option<Body>) -> Result<ResponseWrapper> {
        if let Some(body) = body {
            self.request = self.request.body(body);
        }
        let url = self.url.clone();
        let request = self.set_header().request;

        print_request_log(request.try_clone().unwrap());

        return match request.send().await {
            Ok(response) => {
                print_response_log(&response);

                let status = response.status();

                // Client or server error.
                if status.is_client_error() || status.is_server_error() {
                    return Err(Error::status(url, status).into());
                }

                Ok(ResponseWrapper { response })
            }
            Err(e) => Err(Error::new(ErrorKind::Request, Some(e)).into()),
        };
    }

    pub async fn send(self) -> Result<ResponseWrapper> {
        self.send_body(None).await
    }

    pub async fn send_text(mut self, text: String) -> Result<ResponseWrapper> {
        self.set_header_if_absent("content-type", "text/plain".to_string());
        self.send_body(Some(Body::from(text))).await
    }

    pub async fn send_form<T>(mut self, form: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_if_absent("content-type", "application/x-www-form-urlencoded".to_string());
        let form = serde_urlencoded::to_string(form).map_err(Error::encode)?;
        self.send_body(Some(Body::from(form))).await
    }

    pub async fn send_json<T>(mut self, json: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_if_absent("content-type", "application/json".to_string());
        let json = serde_json::to_string(json).map_err(Error::encode)?;
        self.send_body(Some(Body::from(json))).await
    }
}

#[async_trait]
impl HttpResponse for ResponseWrapper {
    fn status(self) -> StatusCode {
        self.response.status()
    }

    async fn text(self) -> Result<String> {
        self.response.text().await.map_err(Error::decode)
    }
}

impl ResponseWrapper {
    pub async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json::<T>().await.map_err(Error::decode)
    }
}
