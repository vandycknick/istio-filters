use std::option::Option::None;
use std::rc::Rc;

use cookie::Cookie;

use chrono::offset::Local;
use chrono::DateTime;

use envoy::extension::filter::http;
use envoy::extension::{HttpFilter, InstanceId, Result};
use envoy::host::{log, Clock, HttpClient};

use super::config::JwtFromCookieConfig;

pub struct JwtFromCookieHttpFilter<'a> {
    config: Rc<JwtFromCookieConfig>,
    instance_id: InstanceId,
    clock: &'a dyn Clock,
    http_client: &'a dyn HttpClient,
}

impl<'a> JwtFromCookieHttpFilter<'a> {
    /// Creates a new instance of Sample HTTP Filter.
    pub fn new(
        config: Rc<JwtFromCookieConfig>,
        instance_id: InstanceId,
        clock: &'a dyn Clock,
        http_client: &'a dyn HttpClient,
    ) -> Self {
        // Inject dependencies on Envoy host APIs
        JwtFromCookieHttpFilter {
            config,
            instance_id,
            clock,
            http_client,
        }
    }
}

impl<'a> HttpFilter for JwtFromCookieHttpFilter<'a> {
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
            .request_header("Cookie")
            .unwrap_or_default()
            .map(|c| c.into_string().unwrap_or_default())
            .unwrap_or_default()
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Cookie::parse_encoded)
            .filter_map(|c| c.ok())
            .find(|c| c.name() == self.config.cookie)
        {
            Some(cookie) => {
                let value = format!("{}{}", self.config.prefix, cookie.value());
                filter_ops.set_request_header(&self.config.header, &value)?;
            }
            None => log::info!("Cookie {} not found or empty!", self.config.cookie),
        };

        Ok(http::FilterHeadersStatus::Continue)
    }
}
