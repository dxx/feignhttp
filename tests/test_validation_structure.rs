#![allow(unused_imports)]
#![allow(dead_code)]

use feignhttp::{feign, Result};

#[test]
fn test_validate_struct_impl() {
    // error: no metadata assign
    //    |  #[feign]
    //    |  ^^^^^^^^

    pub struct Http {}

    // #[feign]
    impl Http {}
}

#[test]
fn test_validate_impl_url() {
    // error: metadata url not specified
    //    |  #[feign(aaa = "http://xxx")]
    //    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    pub struct Http {}

    // #[feign(aaa = "http://xxx")]
    impl Http {}
}

#[test]
fn test_validate_impl_url2() {
    // error: metadata url not specified
    //    |  #[feign(path = "http://xxx")]
    //    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    pub struct Http {}

    // #[feign(path = "http://xxx")]
    impl Http {}
}

#[test]
fn test_validate_impl_method() {
    // error: unknown request method marker: req
    //    |  #[req]
    //    |    ^^^

    pub struct Http {}

    // #[feign("http://xxx")]
    impl Http {
        // #[req]
        pub async fn get() {}
    }
}

#[test]
fn test_validate_impl_method_path() {
    // error: metadata path not specified or must be the first
    //    |  #[get(aaa = "/aaa")]
    //    |        ^^^^^^^^^^^^

    pub struct Http {}

    // #[feign("http://xxx")]
    impl Http {
        // #[get(aaa = "/aaa")]
        // pub async fn get() -> Result<String> {}
    }
}
