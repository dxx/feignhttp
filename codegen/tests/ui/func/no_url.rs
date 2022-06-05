use feignhttp_codegen::get;

// error: metadata url not specified
//    |  #[get(aaa = "http://xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^

#[get(aaa = "http://xxx")]
fn send_get() {}

fn main() {}
