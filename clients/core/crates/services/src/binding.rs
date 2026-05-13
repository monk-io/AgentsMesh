// proto.binding.v1.BindingService Connect-RPC bridge. The legacy REST
// surface was retired; this service is a thin owner of the ApiClient that
// forwards to the api-client `*_connect` methods (see binding_connect.rs
// for the bridge methods).

use std::sync::Arc;

use agentsmesh_api_client::ApiClient;

pub struct BindingService {
    client: Arc<ApiClient>,
}

impl BindingService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    /// Crate-local accessor used by binding_connect.rs to forward to the
    /// underlying api-client `*_connect` methods.
    pub(crate) fn client(&self) -> &ApiClient {
        &self.client
    }
}
