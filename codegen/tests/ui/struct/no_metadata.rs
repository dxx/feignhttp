use feignhttp_codegen::feign;

// error: no metadata assign
//    |  #[feign]
//    |  ^^^^^^^^

struct Http;

#[feign]
impl Http {}

fn main() {}
