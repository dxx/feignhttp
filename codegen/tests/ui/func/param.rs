use feignhttp_codegen::get;

// error: unsupported param parameter: `a: Vec < String >`
//    |  pub async fn send_get(#[param] a: Vec<String>) -> Result<String> {}
//    |                                 ^^^^^^^^^^^^^^

#[get("http://xxx")]
pub async fn send_get(#[param] a: Vec<String>) -> Result<String> {}

fn main() {}
