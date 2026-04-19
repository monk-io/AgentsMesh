use std::collections::HashMap;

use crate::ApiClient;
use crate::error::ApiError;
use crate::request::RequestOptions;
use agentsmesh_types::*;
use reqwest::Method;

impl ApiClient {
    pub async fn request_binding(
        &self,
        data: &CreateBindingRequest,
        pod_key: Option<&str>,
    ) -> Result<Binding, ApiError> {
        let mut opts = RequestOptions {
            body: Some(serde_json::to_value(data)?),
            ..Default::default()
        };
        if let Some(key) = pod_key {
            let mut headers = HashMap::new();
            headers.insert("X-Pod-Key".to_string(), key.to_string());
            opts.headers = Some(headers);
        }
        self.request(Method::POST, &self.org_path("/bindings"), opts)
            .await
    }

    pub async fn accept_binding(
        &self,
        data: &AcceptBindingRequest,
    ) -> Result<Binding, ApiError> {
        self.post(&self.org_path("/bindings/accept"), data).await
    }

    pub async fn reject_binding(
        &self,
        data: &RejectBindingRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path("/bindings/reject"), data).await
    }

    pub async fn request_binding_scopes(
        &self,
        binding_id: i64,
        data: &RequestScopesBody,
    ) -> Result<Binding, ApiError> {
        self.post(
            &self.org_path(&format!("/bindings/{binding_id}/scopes")),
            data,
        )
        .await
    }

    pub async fn approve_binding_scopes(
        &self,
        binding_id: i64,
        data: &ApproveScopesBody,
    ) -> Result<Binding, ApiError> {
        self.post(
            &self.org_path(&format!("/bindings/{binding_id}/scopes/approve")),
            data,
        )
        .await
    }

    pub async fn unbind(&self, data: &UnbindRequest) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path("/bindings/unbind"), data).await
    }

    pub async fn list_bindings(
        &self,
        status: Option<&str>,
    ) -> Result<BindingListResponse, ApiError> {
        let mut path = self.org_path("/bindings");
        if let Some(s) = status {
            path = format!("{path}?status={s}");
        }
        self.get(&path).await
    }

    pub async fn get_pending_bindings(&self) -> Result<BindingListResponse, ApiError> {
        self.get(&self.org_path("/bindings/pending")).await
    }

    pub async fn get_bound_pods(&self) -> Result<BoundPodsResponse, ApiError> {
        self.get(&self.org_path("/bindings/pods")).await
    }

    pub async fn check_binding(&self, target_pod: &str) -> Result<Binding, ApiError> {
        self.get(&self.org_path(&format!("/bindings/check/{target_pod}")))
            .await
    }
}
