use std::fmt;

/// Http request method
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

/// Http request content
#[derive(PartialEq)]
pub enum Content {
    HEADER,
    PATH,
    QUERY,
    FORM,
    BODY,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self {
            Content::HEADER => "header",
            Content::PATH => "path",
            Content::QUERY => "query",
            Content::FORM => "form",
            Content::BODY => "body",
        };
        write!(f, "{}", content)
    }
}

impl Content {
    pub fn from_str(content: &str) -> Result<Content, String> {
        match content {
            "header" | "HEADER" => Ok(Content::HEADER),
            "path" | "PATH" => Ok(Content::PATH),
            "query" | "QUERY" => Ok(Content::QUERY),
            "form" | "FORM" => Ok(Content::FORM),
            "body" | "BODY" => Ok(Content::BODY),
            _ => Err("unknown request content marker: ".to_string() + content),
        }
    }
}
