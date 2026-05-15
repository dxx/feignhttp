use feignhttp_codegen::feign;

// error: metadata url not specified
//    |  #[feign(aaa = "http://xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

#[feign(aaa = "http://xxx")]
pub trait Http {}

fn main() {}
