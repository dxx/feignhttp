use std::collections::HashMap;

pub fn replace(placeholder: &str, map: &HashMap<&str, String>) -> String {
    let mut placeholder = placeholder.to_string();
    for (k, v) in map {
        placeholder = placeholder.replace(format!("{{{}}}", k).as_str(), &v);
    }
    placeholder
}
