use std::collections::HashMap;
use crate::error::Result;

pub trait FeignContext {

    fn param_map(&self) -> HashMap<&str, String>;

    fn path_map(&self) -> HashMap<&str, String>;

    fn header_map(&self) -> Result<HashMap<&str, String>>;

    fn query_map(&self) -> Result<Vec<(&str, String)>>;

}
