#[allow(unused_imports)]

use feignhttp::get;

#[test]
fn test_validate_no_url () {
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
fn test_validate_async () {
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
fn test_validate_arg() {
    // error: unknown content marker: aaa
    //    |  pub async fn get(#[aaa] a: i32) -> Result<String, String> {}
    //    |                     ^^^

    // #[get("http://xxx")]
    // pub async fn get(#[aaa] a: i32) -> Result<String, String> {}
}

#[test]
fn test_validate_arg2() {
    // error: unknown content marker: bbb
    //    |  pub async fn get(a: i32, #[bbb] b: i32) -> Result<String, String> {}
    //    |                             ^^^

    // #[get("http://xxx")]
    // pub async fn get(a: i32, #[bbb] b: i32) -> Result<String, String> {}
}
