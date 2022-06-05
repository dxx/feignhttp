use feignhttp_codegen::get;

// error: return value must be Result
//    |  pub async fn send_get() -> i32 {}
//    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^

#[get("http://xxx")]
pub async fn send_get() -> i32 {}

fn main() {}
