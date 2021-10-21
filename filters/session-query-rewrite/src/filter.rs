use std::rc::Rc;

use chrono::offset::Local;
use chrono::DateTime;

use ::http::Uri;
use envoy::extension::filter::http;
use envoy::extension::{HttpFilter, InstanceId, Result};
use envoy::host::{log, Clock};

use super::config::SessionQueryRewriteConfig;
use super::querystring::querify;

pub struct SessionQueryRewriteHttpFilter<'a> {
    config: Rc<SessionQueryRewriteConfig>,
    instance_id: InstanceId,
    clock: &'a dyn Clock,
    has_match: bool,
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
            has_match: false,
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
                log::info!("=== Logging incoming headers ===");
                for (name, value) in headers {
                    log::info!("Header: {}={}", name, value);
                }
                log::info!("================================");
            }
            Err(err) => {
                log::error!("something gone wrong {}", err);
            }
        }

        // Always remove any rewrite headers if they where set by an origin
        filter_ops
            .remove_request_header(self.config.host_rewrite_header.as_str())
            .unwrap_or_default();

        let authority = filter_ops
            .request_header(":authority")
            .unwrap_or_default()
            .map(|authority| format!("{}", authority))
            .unwrap_or_default();

        let path = filter_ops
            .request_header(":path")
            .unwrap_or_default()
            .map(|authority| format!("{}", authority))
            .unwrap_or_default();

        if authority == self.config.vhost && self.config.routes.iter().any(|r| path.starts_with(r))
        {
            let uri = path.parse::<Uri>().unwrap_or_default();

            match uri
                .query()
                .map(|query| querify(query))
                .unwrap_or_default()
                .into_iter()
                .filter(|part| part.0 == "sid")
                .nth(0)
            {
                Some(sid) => {
                    self.has_match = true;
                    let host = format!("{}.sessions.sessions.svc.cluster.local", sid.1);

                    filter_ops
                        .set_request_header(self.config.host_rewrite_header.as_str(), &host)
                        .unwrap_or_default();
                }

                None => {
                    log::info!("No sid found, not setting rewrite header");
                }
            }
        } else {
            log::info!("Vhost or path does not match, not extracting sid to rewrite!")
        }

        Ok(http::FilterHeadersStatus::Continue)
    }

    fn on_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
        ops: &dyn http::ResponseHeadersOps,
    ) -> Result<http::FilterHeadersStatus> {
        if self.has_match {
            let status_code = ops
                .response_header(":status")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .unwrap_or_default();

            if !status_code.is_empty() && status_code == "503" {
                ops.set_response_header(":status", "403")
                    .unwrap_or_default();
            }
        }

        Ok(http::FilterHeadersStatus::Continue)
    }
}
