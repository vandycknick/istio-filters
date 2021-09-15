use envoy::extension::{entrypoint, Module, Result};

use session_query_rewrite::SessionQueryRewriteHttpFilterFactory;

// Generate the `_start` function that will be called by `Envoy` to let
// WebAssembly module initialize itself.
entrypoint! { initialize }

/// Does one-time initialization.
///
/// Returns a registry of extensions provided by this module.
fn initialize() -> Result<Module> {
    Module::new().add_http_filter(|_instance_id| SessionQueryRewriteHttpFilterFactory::default())
}
