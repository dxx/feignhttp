use std::fmt;

/// Http request method.
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    pub fn to_str(&self) -> &str {
        match *self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        }
    }
    pub fn from_str(str: &str) -> Result<Method, String> {
        match str {
            "get" | "GET" => Ok(Method::GET),
            "post" | "POST" => Ok(Method::POST),
            "put" | "PUT" => Ok(Method::PUT),
            "delete" | "DELETE" => Ok(Method::DELETE),
            _ => Err("unknown request method marker: ".to_string() + str),
        }
    }
}

/// Arg type.
#[derive(PartialEq)]
pub enum ArgType {
    HEADER,
    PATH,
    QUERY,
    FORM,
    BODY,
    PARAM,
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match self {
            ArgType::HEADER => "header",
            ArgType::PATH => "path",
            ArgType::QUERY => "query",
            ArgType::FORM => "form",
            ArgType::BODY => "body",
            ArgType::PARAM => "PARAM",
        };
        write!(f, "{}", t)
    }
}

impl ArgType {
    pub fn from_str(arg_type: &str) -> Result<ArgType, String> {
        match arg_type {
            "header" | "HEADER" => Ok(ArgType::HEADER),
            "path" | "PATH" => Ok(ArgType::PATH),
            "query" | "QUERY" => Ok(ArgType::QUERY),
            "form" | "FORM" => Ok(ArgType::FORM),
            "body" | "BODY" => Ok(ArgType::BODY),
            "param" | "PARAM" => Ok(ArgType::PARAM),
            _ => Err("unknown arg type: ".to_string() + arg_type),
        }
    }
}
