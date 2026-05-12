use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use agentsmesh_types::proto_ticket_relations_v1 as tr_proto;
use prost::Message;

pub struct TicketRelationsService {
    client: Arc<ApiClient>,
}

impl TicketRelationsService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list_relations(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_relations(slug)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_relation(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: CreateTicketRelationRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .create_ticket_relation(slug, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete_relation(&self, slug: &str, relation_id: i64) -> Result<(), String> {
        self.client
            .delete_ticket_relation(slug, relation_id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_commits(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_commits(slug)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn link_commit(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: LinkTicketCommitRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .link_ticket_commit(slug, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn unlink_commit(&self, slug: &str, commit_id: i64) -> Result<(), String> {
        self.client
            .unlink_ticket_commit(slug, commit_id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    pub async fn list_merge_requests(&self, slug: &str) -> Result<String, String> {
        let resp = self.client
            .list_ticket_merge_requests(slug)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn list_comments(
        &self, slug: &str, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_ticket_comments(slug, limit, offset)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_comment(&self, slug: &str, json: &str) -> Result<String, String> {
        let req: CreateTicketCommentRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .create_ticket_comment(slug, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_comment(
        &self, slug: &str, comment_id: i64, json: &str,
    ) -> Result<String, String> {
        let req: UpdateTicketCommentRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client
            .update_ticket_comment(slug, comment_id, &req)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn delete_comment(&self, slug: &str, comment_id: i64) -> Result<(), String> {
        self.client
            .delete_ticket_comment(slug, comment_id)
            .await.map_err(crate::wire)?;
        Ok(())
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes and returns
    // prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
    // surface (conventions §2.5). Caller (TS) encodes via
    // @bufbuild/protobuf .toBinary() and decodes via .fromBinary().

    pub async fn list_relations_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::ListRelationsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_relations request: {e}"))?;
        let resp = self.client.list_relations_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_relation_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::CreateRelationRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_relation request: {e}"))?;
        let resp = self.client.create_relation_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_relation_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::DeleteRelationRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_relation request: {e}"))?;
        let resp = self.client.delete_relation_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_merge_requests_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::ListMergeRequestsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_merge_requests request: {e}"))?;
        let resp = self.client.list_ticket_merge_requests_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_commits_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::ListCommitsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_commits request: {e}"))?;
        let resp = self.client.list_ticket_commits_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn link_commit_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::LinkCommitRequest::decode(request_bytes)
            .map_err(|e| format!("decode link_commit request: {e}"))?;
        let resp = self.client.link_commit_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn unlink_commit_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::UnlinkCommitRequest::decode(request_bytes)
            .map_err(|e| format!("decode unlink_commit request: {e}"))?;
        let resp = self.client.unlink_commit_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_comments_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::ListCommentsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_comments request: {e}"))?;
        let resp = self.client.list_comments_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::CreateCommentRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_comment request: {e}"))?;
        let resp = self.client.create_comment_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::UpdateCommentRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_comment request: {e}"))?;
        let resp = self.client.update_comment_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_comment_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = tr_proto::DeleteCommentRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_comment request: {e}"))?;
        let resp = self.client.delete_comment_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
