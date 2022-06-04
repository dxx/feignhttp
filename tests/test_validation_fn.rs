#![allow(unused_imports)]

use feignhttp::{get, post, Result};

#[test]
fn test_validate_no_url() {
    // error: no metadata assign
    //    |  #[get]
    //    |  ^^^^^^^^

    // #[get]
    // pub fn get() {}
}

#[test]
fn test_validate_url() {
    // error: metadata url not specified
    //    |  #[get(aaa = "http://xxx")]
    //    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^

    // #[get(aaa = "http://xxx")]
    // pub fn get() {}
}

#[test]
fn test_validate_url2() {
    // error: metadata url not specified
    //    |  #[get(path = "/xxx")]
    //    |  ^^^^^^^^^^^^^^^^^^^^^

    // #[get(path = "/xxx")]
    // pub fn get() {}
}

#[test]
fn test_validate_invalid_url() {
    // error: metadata url is invalid
    //    |  #[get(url + "/aaa")]
    //    |  ^^^^^^^^^^^^^^^^^^^^

    // let url = "http://xxx";

    // #[get(url + "/aaa")]
    // pub fn get() {}
}

#[test]
fn test_validate_async() {
    // error: only support async fn
    //    |  pub fn get() {}
    //    |      ^^

    // #[get("http://xxx")]
    // pub fn get() {}
}

#[test]
fn test_validate_return_value() {
    // error: function must have a return value
    //    |  pub async fn get() {}
    //    |      ^^^^^^^^^^^^^^

    // #[get("http://xxx")]
    // pub async fn get() {}
}

#[test]
fn test_validate_return_value2() {
    // error: return value must be Result
    //    |  pub async fn get() -> i32 {}
    //    |      ^^^^^^^^^^^^^^^^^^^^^

    // #[get("http://xxx")]
    // pub async fn get() -> i32 {}
}

#[test]
fn test_validate_attr() {
    // error: unknown arg type: aaa
    //    |  pub async fn get(#[aaa] a: i32) -> Result<String> {}
    //    |                     ^^^

    // #[get("http://xxx")]
    // pub async fn get(#[aaa] a: i32) -> Result<String> {}
}

#[test]
fn test_validate_attr2() {
    // error: unknown arg type: bbb
    //    |  pub async fn get(a: i32, #[bbb] b: i32) -> Result<String> {}
    //    |                             ^^^

    // #[get("http://xxx")]
    // pub async fn get(a: i32, #[bbb] b: i32) -> Result<String> {}
}

#[test]
fn test_validate_param() {
    // error: unknown arg type: bbb
    //    |  pub async fn get(#[param] a: Vec<String>) -> Result<String> {}
    //    |                            ^^^^^^^^^^^^^^

    // #[get("http://xxx")]
    // pub async fn get(#[param] a: Vec<String>) -> Result<String> {}
}

#[test]
fn test_validate_form() {
    // error: one form parameter only supports scalar types, &str, String or struct
    //    |  pub async fn get(#[form] s: &String) -> Result<String> {}
    //    |                           ^^^^^^^^^^

    // #[post("http://xxx")]
    // pub async fn get(#[form] s: &String) -> Result<String> {}
}

#[test]
fn test_validate_form2() {
    // error: two or more form parameters only supports scalar types, &str or String
    //    |  pub async fn get(#[form] a: i32, #[form] b: &i32, #[form] f: F) -> Result<String> {}
    //    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    // struct F;

    // #[post("http://xxx")]
    // pub async fn get(#[form] a: i32, #[form] b: &i32, #[form] f: F) -> Result<String> {}
}

#[test]
fn test_validate_body() {
    // error: request must have only one body
    //    |  pub async fn get(#[body] b: B, #[body] b2: B) -> Result<String> {}
    //    |                           ^^^^^^^^^^^^^^^^^^^

    // struct B;

    // #[post("http://xxx")]
    // pub async fn get(#[body] b: B, #[body] b2: B) -> Result<String> {}
}

#[test]
fn test_validate_body_form() {
    // error: request must have only one of body or form
    //    |  pub async fn get(#[body] b: S, #[form] f: S) -> Result<String> {}
    //    |                           ^^^^^^^^^^^^^^^^^^

    // struct S;

    // #[post("http://xxx")]
    // pub async fn get(#[body] b: S, #[form] f: S) -> Result<String> {}
}
