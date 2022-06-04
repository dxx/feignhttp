use http::StatusCode;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt;
use url::Url;

/// A `Result` alias.
pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Build, // Indicates an error occurred while build http client.
    Config, // Indicates an error occurred while crate http config.
    Encode, // Indicates an error occurred while encode request body.
    Decode, // Indicates an error occurred while encode response body.
    Request, // Indicates an error occurred while request target url.
    Status(StatusCode), // Indicates an error occurred when the http status is not ok.
}

/// The errors that may occur when processing a request.
pub struct Error {
    inner: Box<Inner>,
}

struct Inner {
    kind: ErrorKind,
    source: Option<BoxError>,
    url: Option<Url>,
}

impl Error {
    pub(crate) fn new<E>(kind: ErrorKind, source: Option<E>) -> Error
        where
            E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
                url: None,
            }),
        }
    }

    pub(crate) fn build<E: Into<BoxError>>(e: E) -> Error {
        Error::new(ErrorKind::Build, Some(e))
    }

    pub(crate) fn config<E: Into<BoxError>>(e: E) -> Error {
        Error::new(ErrorKind::Config, Some(e))
    }

    pub(crate) fn decode<E: Into<BoxError>>(e: E) -> Error {
        Error::new(ErrorKind::Decode, Some(e))
    }

    pub(crate) fn encode<E: Into<BoxError>>(e: E) -> Error {
        Error::new(ErrorKind::Encode, Some(e))
    }

    pub(crate) fn status(url: Url, status: StatusCode) -> Self {
        Error::new(ErrorKind::Status(status), None::<Error>).with_url(url)
    }

    pub(crate) fn with_url(mut self, url: Url) -> Self {
        self.inner.url = Some(url);
        self
    }

    pub fn error_kind(&self) -> ErrorKind {
        self.inner.kind.clone()
    }

    pub fn is_build_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Build)
    }

    pub fn is_config_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Config)
    }

    pub fn is_encode_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Encode)
    }

    pub fn is_decode_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Decode)
    }

    pub fn is_request_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Request)
    }

    pub fn is_status_error(&self) -> bool {
        matches!(self.inner.kind, ErrorKind::Status(_))
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("feignhttp::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref url) = self.inner.url {
            builder.field("url", url);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.kind {
            ErrorKind::Build => f.write_str("error build client")?,
            ErrorKind::Config => f.write_str("error create config")?,
            ErrorKind::Request => f.write_str("error sending request")?,
            ErrorKind::Encode => f.write_str("error encoding request body")?,
            ErrorKind::Decode => f.write_str("error decoding response body")?,
            ErrorKind::Status(ref status_code) => {
                let prefix = if status_code.is_client_error() {
                    "HTTP status client error"
                } else {
                    debug_assert!(status_code.is_server_error());
                    "HTTP status server error"
                };
                write!(f, "{} ({})", prefix, status_code)?;
            }
        }

        if let Some(ref url) = self.inner.url {
            write!(f, " for url ({})", url.as_str())?;
        }

        if let Some(ref e) = self.inner.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}
