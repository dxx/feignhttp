use feignhttp_codegen::feign;

// error: metadata path not specified or must be the first
//    |  #[get(aaa = "/aaa")]
//    |        ^^^^^^^^^^^^

struct Http;

#[feign("http://xxx")]
impl Http {
    #[get(aaa = "/aaa")]
    pub async fn get() -> Result<String> {}
}

fn main() {}
