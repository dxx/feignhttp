use crate::error::Result;
use std::collections::HashMap;

pub trait FeignContext: Send + Sync {
    fn param_map(&self) -> HashMap<String, String>;

    fn path_map(&self) -> HashMap<String, String>;

    fn header_map(&self) -> Result<HashMap<String, String>>;

    fn query_map(&self) -> Result<Vec<(String, String)>>;
}
