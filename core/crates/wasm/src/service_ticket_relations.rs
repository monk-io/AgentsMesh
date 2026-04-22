use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTicketRelationsService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmTicketRelationsService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list_relations(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_relations(slug)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn create_relation(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: CreateTicketRelationRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client
            .create_ticket_relation(slug, &req)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn delete_relation(&self, slug: &str, relation_id: i64) -> Result<(), String> {
        self.client
            .delete_ticket_relation(slug, relation_id)
            .await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }

    pub async fn list_commits(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_commits(slug)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn link_commit(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: LinkTicketCommitRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client
            .link_ticket_commit(slug, &req)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn unlink_commit(&self, slug: &str, commit_id: i64) -> Result<(), String> {
        self.client
            .unlink_ticket_commit(slug, commit_id)
            .await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }

    pub async fn list_merge_requests(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_merge_requests(slug)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn list_comments(
        &self, slug: &str, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_ticket_comments(slug, limit, offset)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn create_comment(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: CreateTicketCommentRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client
            .create_ticket_comment(slug, &req)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn update_comment(
        &self, slug: &str, comment_id: i64, json: &str,
    ) -> Result<String, String> {
        let req: UpdateTicketCommentRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client
            .update_ticket_comment(slug, comment_id, &req)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn delete_comment(&self, slug: &str, comment_id: i64) -> Result<(), String> {
        self.client
            .delete_ticket_comment(slug, comment_id)
            .await.map_err(agentsmesh_services::wire)?;
        Ok(())
    }
}
