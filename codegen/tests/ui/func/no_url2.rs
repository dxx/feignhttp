use feignhttp_codegen::get;

// error: metadata url not specified
//    |  #[get(path = "/xxx")]
//    |  ^^^^^^^^^^^^^^^^^^^^^

#[get(path = "/xxx")]
fn send_get() {}

fn main() {}
