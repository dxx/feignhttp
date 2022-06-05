use feignhttp_codegen::feign;

// error: metadata url not specified
//    |  #[feign(path = "http://xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

struct Http;

#[feign(path = "http://xxx")]
impl Http {}

fn main() {}
