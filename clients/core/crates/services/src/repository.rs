use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::proto_repository_v1 as repo_proto;
use agentsmesh_types::*;
use prost::Message;

pub struct RepositoryService {
    client: Arc<ApiClient>,
}

impl RepositoryService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<String, String> {
        let req = repo_proto::ListRepositoriesRequest {
            org_slug: self.client.current_org_slug(),
            offset: None,
            limit: None,
        };
        let resp = self.client.list_repositories_connect(&req).await.map_err(crate::wire)?;
        let repositories: Vec<Repository> = resp.items.into_iter().map(crate::proto_convert::repository::from_proto).collect();
        let envelope = serde_json::json!({ "repositories": repositories });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get(&self, id: i64) -> Result<String, String> {
        let req = repo_proto::GetRepositoryRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        let resp = self.client.get_repository_connect(&req).await.map_err(crate::wire)?;
        let repository = crate::proto_convert::repository::from_proto(resp);
        serde_json::to_string(&repository).map_err(crate::wire)
    }

    pub async fn create(&self, json: &str) -> Result<String, String> {
        let req_legacy: CreateRepositoryRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = repo_proto::CreateRepositoryRequest {
            org_slug: self.client.current_org_slug(),
            provider_type: req_legacy.provider_type.unwrap_or_default(),
            provider_base_url: req_legacy.provider_base_url.unwrap_or_default(),
            http_clone_url: req_legacy.http_clone_url,
            ssh_clone_url: req_legacy.ssh_clone_url,
            external_id: req_legacy.external_id.unwrap_or_default(),
            name: req_legacy.name,
            slug: req_legacy.slug.unwrap_or_default(),
            default_branch: req_legacy.default_branch,
            ticket_prefix: req_legacy.ticket_prefix,
            visibility: req_legacy.visibility,
        };
        let resp = self.client.create_repository_connect(&req).await.map_err(crate::wire)?;
        let repository = crate::proto_convert::repository::from_proto(resp);
        serde_json::to_string(&repository).map_err(crate::wire)
    }

    pub async fn update(&self, id: i64, json: &str) -> Result<String, String> {
        let req_legacy: UpdateRepositoryRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = repo_proto::UpdateRepositoryRequest {
            org_slug: self.client.current_org_slug(),
            id,
            name: req_legacy.name,
            default_branch: req_legacy.default_branch,
            ticket_prefix: req_legacy.ticket_prefix,
            is_active: req_legacy.is_active,
            http_clone_url: req_legacy.http_clone_url,
            ssh_clone_url: req_legacy.ssh_clone_url,
        };
        let resp = self.client.update_repository_connect(&req).await.map_err(crate::wire)?;
        let repository = crate::proto_convert::repository::from_proto(resp);
        serde_json::to_string(&repository).map_err(crate::wire)
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        let req = repo_proto::DeleteRepositoryRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        self.client.delete_repository_connect(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_branches(&self, id: i64) -> Result<String, String> {
        let req = repo_proto::ListRepositoryBranchesRequest {
            org_slug: self.client.current_org_slug(),
            id,
            // Proto carries access_token in the body; the REST surface
            // bound it to a session cookie. Connect handler still reads
            // the same auth, so empty string here is acceptable — backend
            // will fall back to the request-context token.
            access_token: String::new(),
        };
        let resp = self.client.list_repository_branches_connect(&req).await.map_err(crate::wire)?;
        let branches: Vec<serde_json::Value> = resp.items.into_iter().map(|b| serde_json::json!({
            "name": b.name,
            // Proto Branch is name-only; legacy `Branch` had is_default/last_commit
            // that the REST handler also left null. Preserve the legacy shape.
            "is_default": serde_json::Value::Null,
            "last_commit": serde_json::Value::Null,
        })).collect();
        let envelope = serde_json::json!({ "branches": branches });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn sync_branches(&self, id: i64, json: &str) -> Result<String, String> {
        let req_legacy: SyncBranchesRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = repo_proto::SyncRepositoryBranchesRequest {
            org_slug: self.client.current_org_slug(),
            id,
            access_token: req_legacy.access_token.unwrap_or_default(),
        };
        let resp = self.client.sync_repository_branches_connect(&req).await.map_err(crate::wire)?;
        let branches: Vec<serde_json::Value> = resp.items.into_iter().map(|b| serde_json::json!({
            "name": b.name,
            "is_default": serde_json::Value::Null,
            "last_commit": serde_json::Value::Null,
        })).collect();
        let envelope = serde_json::json!({ "branches": branches });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn register_webhook(&self, id: i64) -> Result<(), String> {
        let req = repo_proto::RegisterRepositoryWebhookRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        self.client.register_repository_webhook_connect(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn delete_webhook(&self, id: i64) -> Result<(), String> {
        let req = repo_proto::DeleteRepositoryWebhookRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        self.client.delete_repository_webhook_connect(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn get_webhook_status(&self, id: i64) -> Result<String, String> {
        let req = repo_proto::GetRepositoryWebhookStatusRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        let resp = self.client.get_repository_webhook_status_connect(&req).await.map_err(crate::wire)?;
        let envelope = serde_json::json!({
            "is_configured": resp.registered,
            "url": resp.webhook_url,
            "events": resp.events,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_webhook_secret(&self, id: i64) -> Result<String, String> {
        let req = repo_proto::GetRepositoryWebhookSecretRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        let resp = self.client.get_repository_webhook_secret_connect(&req).await.map_err(crate::wire)?;
        let envelope = serde_json::json!({ "secret": resp.webhook_secret });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn list_merge_requests(
        &self, id: i64, branch: Option<String>, state: Option<String>,
    ) -> Result<String, String> {
        let req = repo_proto::ListRepositoryMergeRequestsRequest {
            org_slug: self.client.current_org_slug(),
            id,
            branch,
            state,
        };
        let resp = self.client.list_repository_merge_requests_connect(&req).await.map_err(crate::wire)?;
        // Legacy MergeRequestListResponse shape: {merge_requests: [{id, title, state, source_branch, target_branch, ...}]}
        let merge_requests: Vec<serde_json::Value> = resp.items.into_iter().map(|m| serde_json::json!({
            "id": m.id,
            "title": m.title,
            "state": m.state,
            "source_branch": m.source_branch,
            "target_branch": m.target_branch,
            // Legacy fields `author` and `created_at` aren't carried by the
            // proto; emit null so callers don't get parse errors.
            "author": serde_json::Value::Null,
            "url": m.mr_url,
            "created_at": serde_json::Value::Null,
        })).collect();
        let envelope = serde_json::json!({ "merge_requests": merge_requests });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn mark_webhook_configured(&self, id: i64) -> Result<(), String> {
        let req = repo_proto::MarkRepositoryWebhookConfiguredRequest {
            org_slug: self.client.current_org_slug(),
            id,
        };
        self.client.mark_repository_webhook_configured_connect(&req).await.map_err(crate::wire)?;
        Ok(())
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes and returns
    // prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
    // surface (conventions §2.5). Caller (TS) encodes via
    // @bufbuild/protobuf .toBinary() and decodes via .fromBinary().

    pub async fn list_repositories_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::ListRepositoriesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_repositories request: {e}"))?;
        let resp = self.client.list_repositories_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_repository_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::GetRepositoryRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_repository request: {e}"))?;
        let resp = self.client.get_repository_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_repository_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::CreateRepositoryRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_repository request: {e}"))?;
        let resp = self.client.create_repository_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_repository_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::UpdateRepositoryRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_repository request: {e}"))?;
        let resp = self.client.update_repository_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_repository_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::DeleteRepositoryRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_repository request: {e}"))?;
        let resp = self.client.delete_repository_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_repository_branches_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::ListRepositoryBranchesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_repository_branches request: {e}"))?;
        let resp = self.client.list_repository_branches_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn sync_repository_branches_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::SyncRepositoryBranchesRequest::decode(request_bytes)
            .map_err(|e| format!("decode sync_repository_branches request: {e}"))?;
        let resp = self.client.sync_repository_branches_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_repository_merge_requests_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::ListRepositoryMergeRequestsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_repository_merge_requests request: {e}"))?;
        let resp = self.client.list_repository_merge_requests_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn register_repository_webhook_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::RegisterRepositoryWebhookRequest::decode(request_bytes)
            .map_err(|e| format!("decode register_repository_webhook request: {e}"))?;
        let resp = self.client.register_repository_webhook_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_repository_webhook_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::DeleteRepositoryWebhookRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_repository_webhook request: {e}"))?;
        let resp = self.client.delete_repository_webhook_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_repository_webhook_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::GetRepositoryWebhookStatusRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_repository_webhook_status request: {e}"))?;
        let resp = self.client.get_repository_webhook_status_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_repository_webhook_secret_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::GetRepositoryWebhookSecretRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_repository_webhook_secret request: {e}"))?;
        let resp = self.client.get_repository_webhook_secret_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn mark_repository_webhook_configured_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = repo_proto::MarkRepositoryWebhookConfiguredRequest::decode(request_bytes)
            .map_err(|e| format!("decode mark_repository_webhook_configured request: {e}"))?;
        let resp = self.client.mark_repository_webhook_configured_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
