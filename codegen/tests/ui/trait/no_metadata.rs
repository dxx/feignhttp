use feignhttp_codegen::feign;

// error: no metadata assign
//    |  #[feign]
//    |  ^^^^^^^^

#[feign]
pub trait Http {}

fn main() {}
