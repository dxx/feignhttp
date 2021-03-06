use reqwest::{RequestBuilder, Response};
use log::debug;

pub fn print_request_log(request: RequestBuilder) {
    if let Ok(request) = request.build() {
        debug!(
            "---> {} {}",
            request.method().to_string(),
            request.url().to_string()
        );
        for (name, value) in request.headers() {
            debug!("{}: {}", name.as_str(), value.to_str().unwrap())
        }
        debug!("");
        let mut body_len = 0;
        if let Some(body) = request.body() {
            let body = body.as_bytes().unwrap();
            debug!("{}", String::from_utf8(body.to_vec()).unwrap());
            body_len = body.len();
        }
        debug!("---> END HTTP ({}-byte body)", body_len);
    }
}

pub fn print_response_log(response: &Response) {
    debug!(
        "<--- {:?} {}",
        response.version(),
        response.status().as_str()
    );
    for (name, value) in response.headers() {
        debug!("{}: {}", name.as_str(), value.to_str().unwrap());
    }
    debug!("<--- END HTTP");
}
