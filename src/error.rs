use http::StatusCode;
use std::error::Error as StdError;
use std::fmt;
use url::Url;

/// A `Result` alias.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub enum ErrorKind {
    Request,
    Decode,
    Status(StatusCode),
}

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

    pub(crate) fn status(url: Url, status: StatusCode) -> Self {
        Error::new(ErrorKind::Status(status), None::<Error>).with_url(url)
    }

    pub(crate) fn with_url(mut self, url: Url) -> Self {
        self.inner.url = Some(url);
        self
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
            ErrorKind::Request => f.write_str("error sending request")?,
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
