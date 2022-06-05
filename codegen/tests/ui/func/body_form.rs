use feignhttp_codegen::post;

// error: request must have only one of body or form
//    |  pub async fn send_post(#[body] b: S, #[form] f: S) -> Result<String> {}
//    |                                 ^^^^^^^^^^^^^^^^^^

struct S;

#[post("http://xxx")]
pub async fn send_post(#[body] b: S, #[form] f: S) -> Result<String> {}

fn main() {}
