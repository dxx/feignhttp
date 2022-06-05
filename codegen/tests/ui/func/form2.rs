use feignhttp_codegen::post;

// error: two or more form parameters only supports scalar types, &str or String
//    |  pub async fn send_post(#[form] a: i32, #[form] b: &i32, #[form] f: F) -> Result<String> {}
//    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

struct F;

#[post("http://xxx")]
pub async fn send_post(#[form] a: i32, #[form] b: &i32, #[form] f: F) -> Result<String> {}

fn main() {}
