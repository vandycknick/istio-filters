use std::convert::TryFrom;

use serde::Deserialize;

use envoy::extension;

#[derive(Deserialize, Debug)]
pub struct SessionQueryRewriteConfig {
    #[serde(default)]
    pub cookie: String,
    #[serde(default)]
    pub header: String,
    #[serde(default)]
    pub prefix: String,
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
            cookie: String::default(),
            header: String::default(),
            prefix: String::default(),
        }
    }
}
