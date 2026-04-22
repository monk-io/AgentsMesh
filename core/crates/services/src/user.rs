use std::sync::Arc;

use agentsmesh_api_client::ApiClient;

pub struct UserApiService {
    client: Arc<ApiClient>,
}

impl UserApiService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_me(&self) -> Result<String, String> {
        let resp = self.client.get_me().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_organizations(&self) -> Result<String, String> {
        let resp = self.client.get_organizations().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }
}
