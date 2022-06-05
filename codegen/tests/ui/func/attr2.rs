use feignhttp_codegen::get;

// error: unknown arg type: bbb
//    |  pub async fn send_get(a: i32, #[bbb] b: i32) -> Result<String> {}
//    |                                  ^^^

#[get("http://xxx")]
pub async fn send_get(a: i32, #[bbb] b: i32) -> Result<String> {}

fn main() {}
