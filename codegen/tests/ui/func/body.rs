use feignhttp_codegen::post;

// error: request must have only one body
//    |  pub async fn send_post(#[body] b: B, #[body] b2: B) -> Result<String> {}
//    |                                 ^^^^^^^^^^^^^^^^^^^

struct B;

#[post("http://xxx")]
pub async fn send_post(#[body] b: B, #[body] b2: B) -> Result<String> {}

fn main() {}
