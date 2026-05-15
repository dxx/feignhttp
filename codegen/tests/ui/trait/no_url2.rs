use feignhttp_codegen::feign;

// error: metadata url not specified
//    |  #[feign(path = "http://xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

#[feign(path = "http://xxx")]
pub trait Http {}

fn main() {}
