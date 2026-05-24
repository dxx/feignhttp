use std::io::Read;

use crate::error::{Error, Result};

pub struct Part {
    name: String,
    value: PartValue,
}

enum PartValue {
    Text(String),
    File(FilePart),
}

pub struct FilePart {
    filename: String,
    content_type: Option<String>,
    data: Vec<u8>,
}

impl Part {
    pub fn text(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: PartValue::Text(value.to_string()),
        }
    }

    pub fn file<T: FileLike>(name: &str, file: T) -> Result<Part> {
        Ok(Part {
            name: name.to_string(),
            value: PartValue::File(FilePart {
                filename: file.filename(),
                content_type: file.content_type(),
                data: file.bytes()?,
            }),
        })
    }

    pub fn file_with_content_type<T: FileLike>(
        name: &str,
        file: T,
        content_type: Option<&str>,
        filename: Option<&str>,
    ) -> Result<Part> {
        Ok(Part {
            name: name.to_string(),
            value: PartValue::File(FilePart {
                filename: filename
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| file.filename()),
                content_type: content_type
                    .map(|s| s.to_string())
                    .or_else(|| file.content_type()),
                data: file.bytes()?,
            }),
        })
    }
}

pub struct MultipartForm {
    parts: Vec<Part>,
}

impl MultipartForm {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add_part(mut self, part: Part) -> Self {
        self.parts.push(part);
        self
    }

    pub fn parts(&self) -> &[Part] {
        &self.parts
    }
}

impl Default for MultipartForm {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FileLike: Send + Sync {
    fn filename(&self) -> String;
    fn content_type(&self) -> Option<String>;
    fn bytes(&self) -> Result<Vec<u8>>;
}

impl FileLike for std::path::PathBuf {
    fn filename(&self) -> String {
        self.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file")
            .to_string()
    }

    fn content_type(&self) -> Option<String> {
        mime_guess::from_path(self).first().map(|m| m.to_string())
    }

    fn bytes(&self) -> Result<Vec<u8>> {
        std::fs::File::open(self)
            .map_err(Error::encode)
            .and_then(|mut file| {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(Error::encode)?;
                Ok(buffer)
            })
    }
}

impl FileLike for std::fs::File {
    fn filename(&self) -> String {
        "file".to_string()
    }

    fn content_type(&self) -> Option<String> {
        None
    }

    fn bytes(&self) -> Result<Vec<u8>> {
        self.try_clone().map_err(Error::encode).and_then(|mut f| {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(Error::encode)?;
            Ok(buffer)
        })
    }
}

impl FileLike for Vec<u8> {
    fn filename(&self) -> String {
        "file".to_string()
    }

    fn content_type(&self) -> Option<String> {
        None
    }

    fn bytes(&self) -> Result<Vec<u8>> {
        Ok(self.clone())
    }
}

pub(crate) fn build_multipart_body(form: &MultipartForm) -> (Vec<u8>, String) {
    let boundary = generate_boundary();
    let mut body = Vec::new();

    for part in form.parts() {
        match &part.value {
            PartValue::Text(text) => {
                body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
                body.extend_from_slice(
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                        part.name
                    )
                    .as_bytes(),
                );
                body.extend_from_slice(text.as_bytes());
                body.extend_from_slice(b"\r\n");
            }
            PartValue::File(file) => {
                body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
                let filename = &file.filename;
                let content_disposition = if let Some(ct) = &file.content_type {
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                        part.name, filename, ct
                    )
                } else {
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: application/octet-stream\r\n\r\n",
                        part.name, filename
                    )
                };
                body.extend_from_slice(content_disposition.as_bytes());
                body.extend_from_slice(&file.data);
                body.extend_from_slice(b"\r\n");
            }
        }
    }

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let content_type = format!("multipart/form-data; boundary={}", boundary);
    (body, content_type)
}

fn generate_boundary() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("-------------------------{:#x}", timestamp)
}
