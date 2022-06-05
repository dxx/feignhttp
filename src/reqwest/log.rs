use log::debug;
use reqwest::{RequestBuilder, Response};

pub fn print_request_log(request: RequestBuilder) {
    if let Ok(request) = request.build() {
        debug!(
            "---> {} {} {:?}",
            request.method().to_string(),
            request.url().to_string(),
            request.version(),
        );
        for (name, value) in request.headers() {
            debug!("{}: {}", name.as_str(), value.to_str().unwrap())
        }
        debug!("");
        let mut body_len = 0;
        if let Some(body) = request.body() {
            let body = body.as_bytes().unwrap();
            match String::from_utf8(body.to_vec()) {
                Ok(s) => debug!("{}", s),
                Err(_) => {}
            }
            body_len = body.len();
        }
        debug!("---> END HTTP ({}-byte body)", body_len);
    }
}

pub fn print_response_log(response: &Response) {
    debug!(
        "<--- {:?} {}",
        response.version(),
        response.status().to_string(),
    );
    for (name, value) in response.headers() {
        debug!("{}: {}", name.as_str(), value.to_str().unwrap());
    }
    debug!("<--- END HTTP");
}
