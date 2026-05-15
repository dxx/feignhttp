use feignhttp_codegen::feign;

// error: metadata path not specified or must be the first
//    |  #[get(aaa = "/aaa")]
//    |        ^^^^^^^^^^^^

#[feign("http://xxx")]
pub trait Http {
    #[get(aaa = "/aaa")]
    async fn get(&self) -> Result<String>;
}

fn main() {}
