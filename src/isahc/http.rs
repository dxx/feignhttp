use crate::{
    error::Error, error::ErrorKind, error::Result, http::HttpConfig, http::HttpRequest,
    http::HttpResponse, map,
};
#[cfg(feature = "log")]
use super::log::{print_request_log, print_response_log};
use async_trait::async_trait;
use http::{request::Builder, Request, Response, StatusCode};
use isahc::{prelude::*, AsyncBody, config::RedirectPolicy};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

/// A wrapper of HTTP request.
pub struct RequestWrapper {
    url: Url,
    headers: HashMap<String, String>,
    request: Builder,
}

/// A wrapper of HTTP response.
pub struct ResponseWrapper {
    response: Response<AsyncBody>,
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
        let request = self.request;
        let uri_ref = request.uri_ref().unwrap();
        let mut url = uri_ref.to_string();
        match uri_ref.query() {
            Some(_) => {
                url.push_str("&");
            }
            None => {
                url.push_str("?");
            }
        }
        let query = serde_urlencoded::to_string(query).unwrap();
        url.push_str(&query);
        self.url = Url::parse(url.as_str()).unwrap();
        self.request = request.uri(url);
        self
    }
}

impl RequestWrapper {
    pub fn build_default(url: &str, method: &str) -> Result<RequestWrapper> {
        let url = Url::parse(url).map_err(Error::build)?;
        let request = Request::builder()
            .uri(url.to_string())
            .method(method.to_uppercase().as_str())
            .redirect_policy(RedirectPolicy::Limit(10));
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
        let mut request = Request::builder();
        if let Some(millisecond) = config.connect_timeout {
            request = request.connect_timeout(Duration::from_millis(millisecond));
        }
        if let Some(millisecond) = config.timeout {
            request = request.timeout(Duration::from_millis(millisecond));
        }
        let url = Url::parse(url).map_err(Error::build)?;
        request = request
            .uri(url.to_string())
            .method(method.to_uppercase().as_str())
            .redirect_policy(RedirectPolicy::Limit(10));
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

    async fn send_body(self, body: Option<Vec<u8>>) -> Result<ResponseWrapper> {
        let url = self.url.clone();
        let mut async_body = AsyncBody::from(());
        if let Some(body) = body.clone() {
            async_body = AsyncBody::from(body);
        }
        let request = self.set_header().request.body(async_body).map_err(Error::build)?;

        #[cfg(feature = "log")]
        print_request_log(&request, body);

        return match request.send_async().await {
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
            Err(e) => Err(Error::new(ErrorKind::Request, Some(e)).with_url(url)),
        };
    }

    pub async fn send(self) -> Result<ResponseWrapper> {
        self.send_body(None).await
    }

    pub async fn send_text(mut self, text: String) -> Result<ResponseWrapper> {
        self.set_header_if_absent("content-type", "text/plain".to_string());
        self.send_body(Some(text.as_bytes().to_vec())).await
    }

    pub async fn send_form<T>(mut self, form: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_if_absent("content-type", "application/x-www-form-urlencoded".to_string());
        let form = serde_urlencoded::to_string(form).map_err(Error::encode)?;
        self.send_body(Some(form.as_bytes().to_vec())).await
    }

    #[cfg(feature = "json")]
    pub async fn send_json<T>(mut self, json: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_if_absent("content-type", "application/json".to_string());
        let json = serde_json::to_string(json).map_err(Error::encode)?;
        self.send_body(Some(json.as_bytes().to_vec())).await
    }

    pub async fn send_vec(mut self, vec: Vec<u8>) -> Result<ResponseWrapper> {
        self.set_header_if_absent("content-type", "application/octet-stream".to_string());
        self.send_body(Some(vec)).await
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

    async fn text(mut self) -> Result<String> {
        self.response.text().await.map_err(Error::decode)
    }

    async fn vec(mut self) -> Result<Vec<u8>> {
        self.response.bytes().await.map_err(Error::decode)
    }
}

impl ResponseWrapper {
    #[cfg(feature = "json")]
    pub async fn json<T>(mut self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Unpin,
    {
        self.response.json::<T>().await.map_err(Error::decode)
    }
}
