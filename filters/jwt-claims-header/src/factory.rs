use std::convert::TryFrom;
use std::rc::Rc;

use envoy::extension::{factory, ConfigStatus, ExtensionFactory, InstanceId, Result};
use envoy::host::{ByteString, Clock, HttpClient};

use super::config::JwtClaimsHeaderConfig;
use super::filter::JwtClaimsHeaderHttpFilter;

/// Factory for creating Sample HTTP Filter instances
/// (one filter instance per HTTP request).
pub struct JwtClaimsHeaderHttpFilterFactory<'a> {
    config: Rc<JwtClaimsHeaderConfig>,
    clock: &'a dyn Clock,
    http_client: &'a dyn HttpClient,
}

impl<'a> JwtClaimsHeaderHttpFilterFactory<'a> {
    /// Creates a new factory.
    pub fn new(clock: &'a dyn Clock, http_client: &'a dyn HttpClient) -> Result<Self> {
        // Inject dependencies on Envoy host APIs
        Ok(JwtClaimsHeaderHttpFilterFactory {
            config: Rc::new(JwtClaimsHeaderConfig::default()),
            clock,
            http_client,
        })
    }

    /// Creates a new factory bound to the actual `Envoy` ABI.
    pub fn default() -> Result<Self> {
        Self::new(Clock::default(), HttpClient::default())
    }
}

impl<'a> ExtensionFactory for JwtClaimsHeaderHttpFilterFactory<'a> {
    type Extension = JwtClaimsHeaderHttpFilter<'a>;

    /// The reference name for Sample HTTP Filter.
    ///
    /// This name appears in `Envoy` configuration as a value of `root_id` field
    /// (also known as `group_name`).
    fn name() -> &'static str {
        "multiplexer.general.http_filter"
    }

    /// Is called when Envoy creates a new Listener that uses Sample HTTP Filter.
    fn on_configure(
        &mut self,
        config: ByteString,
        _ops: &dyn factory::ConfigureOps,
    ) -> Result<ConfigStatus> {
        let config = if config.is_empty() {
            JwtClaimsHeaderConfig::default()
        } else {
            JwtClaimsHeaderConfig::try_from(config.as_bytes())?
        };
        self.config = Rc::new(config);
        Ok(ConfigStatus::Accepted)
    }

    /// Is called to create a unique instance of Sample HTTP Filter
    /// for each HTTP request.
    fn new_extension(&mut self, instance_id: InstanceId) -> Result<Self::Extension> {
        Ok(JwtClaimsHeaderHttpFilter::new(
            Rc::clone(&self.config),
            instance_id,
            self.clock,
            self.http_client,
        ))
    }
}
