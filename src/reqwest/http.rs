use crate::{
    error::{Error, ErrorKind, Result},
    http::{HttpConfig, HttpRequest, HttpResponse},
    map,
};
#[cfg(feature = "log")]
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
    fn headers(mut self, headers: HashMap<&str, String>) -> Self {
        for (k, v) in headers {
            self.headers.insert(k.to_lowercase(), v);
        }
        self
    }

    fn query(mut self, query: Vec<(&str, String)>) -> Self {
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
    pub fn build_default(url: &str, method: &str) -> Result<RequestWrapper> {
        let url = Url::from_str(url).map_err(Error::build)?;
        let request = Client::new().request(
            Method::from_str(method.to_uppercase().as_str()).map_err(Error::build)?,
            url.clone(),
        );
        Ok(RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign HTTP".to_string()),
            request,
        })
    }

    pub fn build_with_config(
        url: &str,
        method: &str,
        config: HttpConfig,
    ) -> Result<RequestWrapper> {
        let mut client = Client::builder();
        if let Some(millisecond) = config.connect_timeout {
            client = client.connect_timeout(Duration::from_millis(millisecond));
        }
        if let Some(millisecond) = config.timeout {
            client = client.timeout(Duration::from_millis(millisecond));
        }
        let url = Url::from_str(url).map_err(Error::build)?;
        let request = client.build().map_err(Error::build)?.request(
            Method::from_str(method.to_uppercase().as_str()).map_err(Error::build)?,
            url.clone(),
        );
        Ok(RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign HTTP".to_string()),
            request,
        })
    }

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

    async fn send_body(self, body: Option<Body>) -> Result<ResponseWrapper> {
        let url = self.url.clone();
        let mut request = self.set_header().request;

        if let Some(body) = body {
            request = request.body(body);
        }

        #[cfg(feature = "log")]
        print_request_log(request.try_clone().unwrap());

        return match request.send().await {
            Ok(response) => {

                #[cfg(feature = "log")]
                print_response_log(&response);

                let status = response.status();

                // Client or server error.
                if status.is_client_error() || status.is_server_error() {
                    return Err(Error::status(url, status));
                }

                Ok(ResponseWrapper { response })
            }
            Err(e) => Err(Error::new(ErrorKind::Request, Some(e))),
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

    #[cfg(feature = "json")]
    pub async fn send_json<T>(mut self, json: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_if_absent("content-type", "application/json".to_string());
        let json = serde_json::to_string(json).map_err(Error::encode)?;
        self.send_body(Some(Body::from(json))).await
    }

    pub async fn send_vec(mut self, vec: Vec<u8>) -> Result<ResponseWrapper> {
        self.set_header_if_absent("content-type", "application/octet-stream".to_string());
        self.send_body(Some(Body::from(vec))).await
    }
}

#[async_trait]
impl HttpResponse for ResponseWrapper {
    fn status(&self) -> StatusCode {
        self.response.status()
    }

    async fn none(self) -> Result<()> {
        Ok(())
    }

    async fn text(self) -> Result<String> {
        self.response.text().await.map_err(Error::decode)
    }

    async fn vec(self) -> Result<Vec<u8>> {
        let by = self.response.bytes().await.map_err(Error::decode)?;
        Ok(by.to_vec())
    }
}

impl ResponseWrapper {
    #[cfg(feature = "json")]
    pub async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json::<T>().await.map_err(Error::decode)
    }
}
