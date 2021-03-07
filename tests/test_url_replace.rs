use std::collections::HashMap;
use feignhttp::util::replace_url;

#[test]
fn test_one_path() {
    let mut map = HashMap::new();
    map.insert("name", "abc".to_string());

    assert_eq!(replace_url("/find/{name}", &map), "/find/abc");
}

#[test]
fn test_multipart_path() {
    let mut map = HashMap::new();
    map.insert("id", "1".to_string());
    map.insert("name", "abc".to_string());

    assert_eq!(replace_url("/{name}/{id}", &map), "/abc/1");
}
