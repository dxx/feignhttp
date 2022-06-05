use feignhttp_codegen::post;

// error: one form parameter only supports scalar types, &str, String or struct
//    |  pub async fn send_post(#[form] s: &String) -> Result<String> {}
//    |                                 ^^^^^^^^^^

#[post("http://xxx")]
pub async fn send_post(#[form] s: &String) -> Result<String> {}

fn main() {}
