use std::option::Option::None;
use std::rc::Rc;
use std::time::Duration;

use chrono::offset::Local;
use chrono::DateTime;

use envoy::extension::filter::http;
use envoy::extension::{HttpFilter, InstanceId, Result};
use envoy::host::{log, Clock, HttpClient, HttpClientRequestHandle, HttpClientResponseOps};

use super::config::SessionQueryRewriteConfig;

pub struct SessionQueryRewriteHttpFilter<'a> {
    config: Rc<SessionQueryRewriteConfig>,
    instance_id: InstanceId,
    clock: &'a dyn Clock,
    http_client: &'a dyn HttpClient,
    active_request: Option<HttpClientRequestHandle>,
}

impl<'a> SessionQueryRewriteHttpFilter<'a> {
    /// Creates a new instance of Sample HTTP Filter.
    pub fn new(
        config: Rc<SessionQueryRewriteConfig>,
        instance_id: InstanceId,
        clock: &'a dyn Clock,
        http_client: &'a dyn HttpClient,
    ) -> Self {
        // Inject dependencies on Envoy host APIs
        SessionQueryRewriteHttpFilter {
            config,
            instance_id,
            clock,
            http_client,
            active_request: None,
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

        filter_ops.request_header("X-InternalSessionHost")?;

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

        filter_ops.set_request_header("X-InternalSessionHost", "httpbin.org")?;

        // filter_ops.set_request_header(":path", "/headers");

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

        // match self.http_client.send_request(
        //     "httpbin.test.svc.cluster.local",
        //     &[
        //         (":method", "GET"),
        //         (":path", "/headers"),
        //         (":authority", "httpbin.test.svc.cluster.local"),
        //     ],
        //     None,
        //     None,
        //     Duration::from_secs(3),
        // ) {
        //     Ok(active_request) => {
        //         self.active_request = Some(active_request);

        //         if let Some(request) = self.active_request {
        //             log::info!(
        //                 "#{} sent authorization request: @{}",
        //                 self.instance_id,
        //                 request,
        //             )
        //         }
        //     }
        //     Err(err) => log::info!("Some error: {}", err),
        // }

        Ok(http::FilterHeadersStatus::Continue)
    }

    fn on_http_call_response(
        &mut self,
        request: HttpClientRequestHandle,
        num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
        filter_ops: &dyn http::Ops,
        http_client_ops: &dyn HttpClientResponseOps,
    ) -> Result<()> {
        log::info!(
            "#{} received response on authorization request: @{}",
            self.instance_id,
            request
        );
        assert!(self.active_request == Some(request));
        self.active_request = None;

        log::info!("     headers[count={}]:", num_headers);
        let response_headers = http_client_ops.http_call_response_headers()?;
        for (name, value) in &response_headers {
            log::info!("       {}: {}", name, value);
        }

        log::info!("#{} resuming http exchange processing", self.instance_id);
        filter_ops.resume_request()?;
        Ok(())
    }
}
