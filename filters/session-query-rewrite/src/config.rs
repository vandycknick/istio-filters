use std::convert::TryFrom;

use serde::Deserialize;

use envoy::extension;

#[derive(Deserialize, Debug)]
pub struct SessionQueryRewriteConfig {
    #[serde(default)]
    pub host_rewrite_header: String,
    #[serde(default)]
    pub routes: Vec<String>,
    #[serde(default)]
    pub vhost: String,
}

impl TryFrom<&[u8]> for SessionQueryRewriteConfig {
    type Error = extension::Error;

    /// Parses filter configuration from JSON.
    fn try_from(value: &[u8]) -> extension::Result<Self> {
        serde_json::from_slice(value).map_err(extension::Error::from)
    }
}

impl Default for SessionQueryRewriteConfig {
    /// Creates the default configuration.
    fn default() -> Self {
        SessionQueryRewriteConfig {
            host_rewrite_header: String::default(),
            routes: Vec::new(),
            vhost: String::default(),
        }
    }
}
