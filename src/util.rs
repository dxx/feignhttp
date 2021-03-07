use std::collections::HashMap;

pub fn replace_url(url: &str, map: &HashMap<&str, String>) -> String {
    let mut url = url.to_string();
    for (k, v) in map {
        url = url.replace(("{".to_string() + k + "}").as_str(), &v);
    }
    url
}
