use crate::error::{Error, Result};
use std::collections::HashMap;

/// Configuration of an HTTP client.
#[derive(Clone, Default)]
pub struct ClientConfig {
    pub connect_timeout: Option<u64>,
    pub timeout: Option<u64>,
    pub read_timeout: Option<u64>,
}

impl ClientConfig {
    pub fn from_map(config_map: HashMap<&str, String>) -> Result<Self> {
        let mut config = ClientConfig::default();
        if let Some(connect_timeout) = config_map.get("connect_timeout") {
            config.connect_timeout = Some(connect_timeout.parse::<u64>().map_err(Error::config)?);
        }
        if let Some(timeout) = config_map.get("timeout") {
            config.timeout = Some(timeout.parse::<u64>().map_err(Error::config)?);
        }
        if let Some(timeout) = config_map.get("read_timeout") {
            config.read_timeout = Some(timeout.parse::<u64>().map_err(Error::config)?);
        }
        Ok(config)
    }
}

/// Configuration of an HTTP request.
#[derive(Clone, Default)]
pub struct RequestConfig {
    pub timeout: Option<u64>,
}

impl RequestConfig {
    pub fn from_map(config_map: HashMap<&str, String>) -> Result<Self> {
        let mut config = RequestConfig::default();
        if let Some(timeout) = config_map.get("timeout") {
            config.timeout = Some(timeout.parse::<u64>().map_err(Error::config)?);
        }
        Ok(config)
    }
}
