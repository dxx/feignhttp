use std::collections::HashMap;
use feignhttp::util::replace;

#[test]
fn test_one_path() {
    let mut map = HashMap::new();
    map.insert("name", "abc".to_string());

    assert_eq!(replace("/find/{name}", &map), "/find/abc");
}

#[test]
fn test_multipart_path() {
    let mut map = HashMap::new();
    map.insert("id", "1".to_string());
    map.insert("name", "abc".to_string());

    assert_eq!(replace("/{name}/{id}", &map), "/abc/1");
}

#[test]
fn test_config() {
    let mut map = HashMap::new();
    map.insert("timeout", "3000".to_string());

    assert_eq!(replace("{timeout}", &map), "3000");
}
