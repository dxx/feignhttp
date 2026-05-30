#[cfg(feature = "log")]
use super::log::{print_request_log, print_response_log};
#[cfg(feature = "multipart")]
use crate::multipart::{MultipartForm, build_multipart_body};
use crate::{
    config::{ClientConfig, RequestConfig},
    error::{Error, ErrorKind, Result},
    http::{Client as ClientTrait, HttpRequest, HttpResponse},
    map,
};
use async_trait::async_trait;
use http::StatusCode;
use reqwest_middleware::RequestBuilder;
use reqwest_middleware::reqwest::{Body, Method, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub struct ClientWrapper {
    reqwest_middleware_client: ClientWithMiddleware,
}

impl ClientTrait for ClientWrapper {
    type Inner = ClientWithMiddleware;

    fn new() -> Result<ClientWrapper> {
        let client = reqwest_middleware::reqwest::Client::builder()
            .build()
            .map_err(Error::build)?;
        let middleware_client = ClientBuilder::new(client).build();
        Ok(ClientWrapper {
            reqwest_middleware_client: middleware_client,
        })
    }

    fn with_config(config: ClientConfig) -> Result<ClientWrapper> {
        let mut client_builder = reqwest_middleware::reqwest::Client::builder();
        if let Some(millisecond) = config.connect_timeout {
            client_builder = client_builder.connect_timeout(Duration::from_millis(millisecond));
        }
        if let Some(millisecond) = config.timeout {
            client_builder = client_builder.timeout(Duration::from_millis(millisecond));
        }
        if let Some(millisecond) = config.read_timeout {
            client_builder = client_builder.read_timeout(Duration::from_millis(millisecond));
        }
        let client = client_builder.build().map_err(Error::build)?;
        let middleware_client = ClientBuilder::new(client).build();
        Ok(ClientWrapper {
            reqwest_middleware_client: middleware_client,
        })
    }

    fn with_client(client: ClientWithMiddleware) -> Result<ClientWrapper> {
        Ok(ClientWrapper {
            reqwest_middleware_client: client,
        })
    }

    fn get_client(&self) -> &ClientWithMiddleware {
        &self.reqwest_middleware_client
    }
}

pub struct RequestWrapper {
    client_wrapper: ClientWrapper,
    url: Url,
    headers: HashMap<String, String>,
    request: RequestBuilder,
    method: Method,
}

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
        if query.is_empty() {
            return self;
        }
        let mut url_str = self.url.to_string();
        let separator = if url_str.contains('?') { "&" } else { "?" };
        let query_str = serde_urlencoded::to_string(query).unwrap();
        url_str.push_str(separator);
        url_str.push_str(&query_str);
        if let Ok(new_url) = Url::parse(&url_str) {
            self.url = new_url.clone();
            self.request = self
                .client_wrapper
                .get_client()
                .request(self.method.clone(), new_url);
        }
        self
    }
}

impl RequestWrapper {
    pub fn new(client_wrapper: ClientWrapper, url: &str, method: &str) -> Result<RequestWrapper> {
        let url = Url::from_str(url).map_err(Error::build)?;
        let method = Method::from_str(method.to_uppercase().as_str()).map_err(Error::build)?;
        let request = client_wrapper
            .get_client()
            .request(method.clone(), url.clone());

        Ok(RequestWrapper {
            client_wrapper,
            url,
            headers: map!("user-agent".to_string() => "Feign HTTP".to_string()),
            request,
            method,
        })
    }

    pub fn with_config(
        client_wrapper: ClientWrapper,
        url: &str,
        method: &str,
        config: RequestConfig,
    ) -> Result<RequestWrapper> {
        let url = Url::from_str(url).map_err(Error::build)?;
        let method = Method::from_str(method.to_uppercase().as_str()).map_err(Error::build)?;
        let mut request = client_wrapper
            .get_client()
            .request(method.clone(), url.clone());
        if let Some(millisecond) = config.timeout {
            request = request.timeout(Duration::from_millis(millisecond));
        }
        Ok(RequestWrapper {
            client_wrapper,
            url,
            headers: map!("user-agent".to_string() => "Feign HTTP".to_string()),
            request,
            method,
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
        if self.headers.get(k).is_none() {
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
        self.set_header_if_absent(
            "content-type",
            "application/x-www-form-urlencoded".to_string(),
        );
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

    #[cfg(feature = "multipart")]
    pub async fn send_multipart(mut self, form: MultipartForm) -> Result<ResponseWrapper> {
        let (body, content_type) = build_multipart_body(&form);
        self.set_header_if_absent("content-type", content_type);
        self.send_body(Some(Body::from(body))).await
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

    #[cfg(feature = "json")]
    async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json::<T>().await.map_err(Error::decode)
    }
}
