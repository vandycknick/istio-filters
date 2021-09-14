use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use base64;

use chrono::offset::Local;
use chrono::DateTime;

use envoy::extension::filter::http;
use envoy::extension::{HttpFilter, InstanceId, Result};
use envoy::host::{log, Clock, HttpClient};

use serde_json::Value;

use super::config::JwtClaimsHeaderConfig;

pub struct JwtClaimsHeaderHttpFilter<'a> {
    config: Rc<JwtClaimsHeaderConfig>,
    instance_id: InstanceId,
    clock: &'a dyn Clock,
    http_client: &'a dyn HttpClient,
}

impl<'a> JwtClaimsHeaderHttpFilter<'a> {
    /// Creates a new instance of Sample HTTP Filter.
    pub fn new(
        config: Rc<JwtClaimsHeaderConfig>,
        instance_id: InstanceId,
        clock: &'a dyn Clock,
        http_client: &'a dyn HttpClient,
    ) -> Self {
        // Inject dependencies on Envoy host APIs
        JwtClaimsHeaderHttpFilter {
            config,
            instance_id,
            clock,
            http_client,
        }
    }
}

impl<'a> HttpFilter for JwtClaimsHeaderHttpFilter<'a> {
    fn on_request_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
        filter_ops: &dyn http::RequestHeadersOps,
    ) -> Result<http::FilterHeadersStatus> {
        let now: DateTime<Local> = self.clock.now()?.into();
        log::info!(
            "#{} new http exchange starts at {} with config: {:?}",
            self.instance_id,
            now.format("%+"),
            self.config,
        );

        match filter_ops
            .request_header(&self.config.header)
            .unwrap_or_default()
        {
            Some(jwt) => {
                log::info!("Jwt header contains {}", jwt);

                let result = match base64::decode(jwt) {
                    Ok(decoded) => decoded,
                    Err(err) => {
                        log::error!("Error decoding base64 encoded JWT: \n {}", err);
                        return Ok(http::FilterHeadersStatus::Continue);
                    }
                };
                let result_str = str::from_utf8(&result).unwrap_or_default();
                let claims = match serde_json::from_str::<HashMap<String, Value>>(result_str) {
                    Ok(claims) => claims,
                    Err(err) => {
                        log::error!("Error parsing jwt claims {} \n: {}", result_str, err);
                        return Ok(http::FilterHeadersStatus::Continue);
                    }
                };
                log::info!("Jwt base64 decoded {}", result_str);
                for (key, value) in claims {
                    let header = format!("{}{}", "X-", key);
                    let value_str = format!("{}", value);
                    log::info!("Adding header {}: {}", header, value_str);
                    filter_ops.set_request_header(&header, &value_str)?;
                }

                return Ok(http::FilterHeadersStatus::Continue);
            }
            None => {
                log::info!("No header found with {}", self.config.header);
                return Ok(http::FilterHeadersStatus::Continue);
            }
        };
    }
}
