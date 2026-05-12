use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::TicketRelationsService;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTicketRelationsService(pub(crate) TicketRelationsService);

#[wasm_bindgen]
impl WasmTicketRelationsService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self(TicketRelationsService::new(client))
    }

    pub async fn list_relations(&self, slug: &str) -> Result<String, String> {
        self.0.list_relations(slug).await
    }

    pub async fn create_relation(&self, slug: &str, json: &str) -> Result<String, String> {
        self.0.create_relation(slug, json).await
    }

    pub async fn delete_relation(&self, slug: &str, relation_id: i64) -> Result<(), String> {
        self.0.delete_relation(slug, relation_id).await
    }

    pub async fn list_commits(&self, slug: &str) -> Result<String, String> {
        self.0.list_commits(slug).await
    }

    pub async fn link_commit(&self, slug: &str, json: &str) -> Result<String, String> {
        self.0.link_commit(slug, json).await
    }

    pub async fn unlink_commit(&self, slug: &str, commit_id: i64) -> Result<(), String> {
        self.0.unlink_commit(slug, commit_id).await
    }

    pub async fn list_merge_requests(&self, slug: &str) -> Result<String, String> {
        self.0.list_merge_requests(slug).await
    }

    pub async fn list_comments(
        &self, slug: &str, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        self.0.list_comments(slug, limit, offset).await
    }

    pub async fn create_comment(&self, slug: &str, json: &str) -> Result<String, String> {
        self.0.create_comment(slug, json).await
    }

    pub async fn update_comment(
        &self, slug: &str, comment_id: i64, json: &str,
    ) -> Result<String, String> {
        self.0.update_comment(slug, comment_id, json).await
    }

    pub async fn delete_comment(&self, slug: &str, comment_id: i64) -> Result<(), String> {
        self.0.delete_comment(slug, comment_id).await
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes (Uint8Array on the JS
    // side) and returns prost-encoded bytes — TS callers encode via
    // @bufbuild/protobuf .toBinary() and decode via .fromBinary(). Suppresses
    // the unused-import lint that triggers when only the legacy REST methods
    // remain in scope (proto types are only referenced through the service).
    #[allow(dead_code)]
    fn _force_types_link(_: &TicketComment) {}

    pub async fn list_relations_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_relations_connect(request_bytes).await
    }

    pub async fn create_relation_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.create_relation_connect(request_bytes).await
    }

    pub async fn delete_relation_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.delete_relation_connect(request_bytes).await
    }

    pub async fn list_merge_requests_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_merge_requests_connect(request_bytes).await
    }

    pub async fn list_commits_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_commits_connect(request_bytes).await
    }

    pub async fn link_commit_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.link_commit_connect(request_bytes).await
    }

    pub async fn unlink_commit_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.unlink_commit_connect(request_bytes).await
    }

    pub async fn list_comments_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_comments_connect(request_bytes).await
    }

    pub async fn create_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.create_comment_connect(request_bytes).await
    }

    pub async fn update_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.update_comment_connect(request_bytes).await
    }

    pub async fn delete_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.0.delete_comment_connect(request_bytes).await
    }
}
