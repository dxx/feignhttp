use feignhttp_codegen::feign;

// error: unknown request method marker: req
//    |  #[req]
//    |    ^^^

#[feign("http://xxx")]
pub trait Http {
    #[req]
    async fn get(&self);
}

fn main() {}
