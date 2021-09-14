use std::convert::TryFrom;

use serde::Deserialize;

use envoy::extension;

#[derive(Deserialize, Debug)]
pub struct JwtClaimsHeaderConfig {
    #[serde(default)]
    pub header: String,
}

impl TryFrom<&[u8]> for JwtClaimsHeaderConfig {
    type Error = extension::Error;

    /// Parses filter configuration from JSON.
    fn try_from(value: &[u8]) -> extension::Result<Self> {
        serde_json::from_slice(value).map_err(extension::Error::from)
    }
}

impl Default for JwtClaimsHeaderConfig {
    /// Creates the default configuration.
    fn default() -> Self {
        JwtClaimsHeaderConfig {
            header: String::default(),
        }
    }
}
