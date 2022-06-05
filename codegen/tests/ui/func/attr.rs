use feignhttp_codegen::get;

// error: unknown arg type: aaa
//    |  pub async fn send_get(#[aaa] a: i32) -> Result<String> {}
//    |                          ^^^

#[get("http://xxx")]
pub async fn send_get(#[aaa] a: i32) -> Result<String> {}

fn main() {}
