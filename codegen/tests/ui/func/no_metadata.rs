use feignhttp_codegen::get;

// error: no metadata assign
//    |  #[get]
//    |  ^^^^^^

#[get]
fn send_get() {}

fn main() {}
