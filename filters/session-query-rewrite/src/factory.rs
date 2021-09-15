use std::convert::TryFrom;
use std::rc::Rc;

use envoy::extension::{factory, ConfigStatus, ExtensionFactory, InstanceId, Result};
use envoy::host::{ByteString, Clock, HttpClient};

use super::config::SessionQueryRewriteConfig;
use super::filter::SessionQueryRewriteHttpFilter;

/// Factory for creating Sample HTTP Filter instances
/// (one filter instance per HTTP request).
pub struct SessionQueryRewriteHttpFilterFactory<'a> {
    config: Rc<SessionQueryRewriteConfig>,
    clock: &'a dyn Clock,
    http_client: &'a dyn HttpClient,
}

impl<'a> SessionQueryRewriteHttpFilterFactory<'a> {
    /// Creates a new factory.
    pub fn new(clock: &'a dyn Clock, http_client: &'a dyn HttpClient) -> Result<Self> {
        // Inject dependencies on Envoy host APIs
        Ok(SessionQueryRewriteHttpFilterFactory {
            config: Rc::new(SessionQueryRewriteConfig::default()),
            clock,
            http_client,
        })
    }

    /// Creates a new factory bound to the actual `Envoy` ABI.
    pub fn default() -> Result<Self> {
        Self::new(Clock::default(), HttpClient::default())
    }
}

impl<'a> ExtensionFactory for SessionQueryRewriteHttpFilterFactory<'a> {
    type Extension = SessionQueryRewriteHttpFilter<'a>;

    /// The reference name for Sample HTTP Filter.
    ///
    /// This name appears in `Envoy` configuration as a value of `root_id` field
    /// (also known as `group_name`).
    fn name() -> &'static str {
        "multiplexer.http_filter.jwt_from_cookie"
    }

    /// Is called when Envoy creates a new Listener that uses Sample HTTP Filter.
    fn on_configure(
        &mut self,
        config: ByteString,
        _ops: &dyn factory::ConfigureOps,
    ) -> Result<ConfigStatus> {
        let config = if config.is_empty() {
            SessionQueryRewriteConfig::default()
        } else {
            SessionQueryRewriteConfig::try_from(config.as_bytes())?
        };
        self.config = Rc::new(config);
        Ok(ConfigStatus::Accepted)
    }

    /// Is called to create a unique instance of Sample HTTP Filter
    /// for each HTTP request.
    fn new_extension(&mut self, instance_id: InstanceId) -> Result<Self::Extension> {
        Ok(SessionQueryRewriteHttpFilter::new(
            Rc::clone(&self.config),
            instance_id,
            self.clock,
            self.http_client,
        ))
    }
}
