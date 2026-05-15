use std::collections::HashMap;

pub trait FeignContext {

    fn param_map(&self) -> HashMap<&str, String>;

    fn header_map(&self) -> HashMap<&str, String>;

    fn path_map(&self) -> HashMap<&str, String>;

    fn query_map(&self) -> Vec<(&str, String)>;

}
