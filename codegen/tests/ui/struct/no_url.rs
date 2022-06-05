use feignhttp_codegen::feign;

// error: metadata url not specified
//    |  #[feign(aaa = "http://xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

struct Http;

#[feign(aaa = "http://xxx")]
impl Http {}

fn main() {}
