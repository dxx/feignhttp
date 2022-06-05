use http::{Request, Response};
use isahc::AsyncBody;
use log::debug;

pub fn print_request_log(request: &Request<AsyncBody>, body: Option<Vec<u8>>) {
    debug!(
        "---> {} {} {:?}",
        request.method().to_string(),
        request.uri().to_string(),
        request.version(),
    );
    for (name, value) in request.headers() {
        debug!("{}: {}", name.as_str(), value.to_str().unwrap())
    }
    debug!("");
    let mut body_len = 0;
    if let Some(vec) = body {
        body_len = vec.len();
        match String::from_utf8(vec) {
            Ok(s) => debug!("{}", s),
            Err(_) => {},
        }
    }
    debug!("---> END HTTP ({}-byte body)", body_len);
}

pub fn print_response_log(response: &Response<AsyncBody>) {
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
