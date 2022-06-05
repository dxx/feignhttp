use feignhttp_codegen::feign;

// error: unknown request method marker: req
//    |  #[req]
//    |    ^^^

struct Http {}

#[feign("http://xxx")]
impl Http {
    #[req]
    pub async fn get() {}
}

fn main() {}
