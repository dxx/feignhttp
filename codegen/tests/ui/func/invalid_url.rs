use feignhttp_codegen::get;

// error: metadata url is invalid
//    |  #[get(url + "/aaa")]
//    |  ^^^^^^^^^^^^^^^^^^^^

const url: &str = "http://xxx";

#[get(url + "/aaa")]
fn send_get() {}

fn main() {}
