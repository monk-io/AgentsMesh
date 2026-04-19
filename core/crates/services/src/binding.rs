use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct BindingService {
    client: Arc<ApiClient>,
}

impl BindingService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn request_binding(
        &self, json: &str, pod_key: Option<String>,
    ) -> Result<String, String> {
        let req: CreateBindingRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client
            .request_binding(&req, pod_key.as_deref())
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn accept_binding(&self, json: &str) -> Result<String, String> {
        let req: AcceptBindingRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client.accept_binding(&req).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn reject_binding(&self, json: &str) -> Result<(), String> {
        let req: RejectBindingRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.client.reject_binding(&req).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn request_scopes(
        &self, binding_id: i64, json: &str,
    ) -> Result<String, String> {
        let req: RequestScopesBody = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client
            .request_binding_scopes(binding_id, &req)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn approve_scopes(
        &self, binding_id: i64, json: &str,
    ) -> Result<String, String> {
        let req: ApproveScopesBody = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let resp = self.client
            .approve_binding_scopes(binding_id, &req)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn unbind(&self, json: &str) -> Result<(), String> {
        let req: UnbindRequest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.client.unbind(&req).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_bindings(&self, status: Option<String>) -> Result<String, String> {
        let resp = self.client
            .list_bindings(status.as_deref())
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_pending_bindings(&self) -> Result<String, String> {
        let resp = self.client.get_pending_bindings().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_bound_pods(&self) -> Result<String, String> {
        let resp = self.client.get_bound_pods().await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn check_binding(&self, target_pod: &str) -> Result<String, String> {
        let resp = self.client.check_binding(target_pod).await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
