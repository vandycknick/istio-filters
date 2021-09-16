use std::rc::Rc;

use chrono::offset::Local;
use chrono::DateTime;

use envoy::extension::filter::http;
use envoy::extension::{HttpFilter, InstanceId, Result};
use envoy::host::{log, Clock};

use super::config::SessionQueryRewriteConfig;

pub struct SessionQueryRewriteHttpFilter<'a> {
    config: Rc<SessionQueryRewriteConfig>,
    instance_id: InstanceId,
    clock: &'a dyn Clock,
}

impl<'a> SessionQueryRewriteHttpFilter<'a> {
    pub fn new(
        config: Rc<SessionQueryRewriteConfig>,
        instance_id: InstanceId,
        clock: &'a dyn Clock,
    ) -> Self {
        SessionQueryRewriteHttpFilter {
            config,
            instance_id,
            clock,
        }
    }
}

impl<'a> HttpFilter for SessionQueryRewriteHttpFilter<'a> {
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

        match filter_ops.request_headers() {
            Ok(headers) => {
                log::info!("Logging available headers:");
                for (name, value) in headers {
                    log::info!("Header: {}={}", name, value);
                }
                log::info!("============");
            }
            Err(err) => {
                log::error!("something gone wrong {}", err);
            }
        }

        filter_ops
            .remove_request_header(self.config.host_rewrite_header.as_str())
            .unwrap_or_default();

        filter_ops
            .set_request_header(self.config.host_rewrite_header.as_str(), "httpbin.org")
            .unwrap_or_default();

        match filter_ops.request_headers() {
            Ok(headers) => {
                log::info!("Logging available headers after update:");
                for (name, value) in headers {
                    log::info!("Header: {}={}", name, value);
                }
                log::info!("============");
            }
            Err(err) => {
                log::error!("something gone wrong {}", err);
            }
        }

        Ok(http::FilterHeadersStatus::Continue)
    }

    fn on_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
        ops: &dyn http::ResponseHeadersOps,
    ) -> Result<http::FilterHeadersStatus> {
        let response_header = ops
            .response_header("status")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .unwrap_or_default();

        if !response_header.is_empty() && response_header == "503" {
            ops.set_response_header("status", "403").unwrap_or_default();
        }

        Ok(http::FilterHeadersStatus::Continue)
    }
}
