use feignhttp_codegen::get;

// error: only support async fn
//    |  pub fn send_get() {}
//    |      ^^

#[get("http://xxx")]
pub fn send_get() {}

fn main() {}
