use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_ticket_relations(
        &self,
        slug: &str,
    ) -> Result<TicketRelationListResponse, ApiError> {
        self.get(&self.org_path(&format!("/tickets/{slug}/relations")))
            .await
    }

    pub async fn create_ticket_relation(
        &self,
        slug: &str,
        data: &CreateTicketRelationRequest,
    ) -> Result<TicketRelation, ApiError> {
        self.post(
            &self.org_path(&format!("/tickets/{slug}/relations")),
            data,
        )
        .await
    }

    pub async fn delete_ticket_relation(
        &self,
        slug: &str,
        relation_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!(
            "/tickets/{slug}/relations/{relation_id}"
        )))
        .await
    }

    pub async fn list_ticket_commits(
        &self,
        slug: &str,
    ) -> Result<TicketCommitListResponse, ApiError> {
        self.get(&self.org_path(&format!("/tickets/{slug}/commits")))
            .await
    }

    pub async fn link_ticket_commit(
        &self,
        slug: &str,
        data: &LinkTicketCommitRequest,
    ) -> Result<TicketCommit, ApiError> {
        self.post(&self.org_path(&format!("/tickets/{slug}/commits")), data)
            .await
    }

    pub async fn unlink_ticket_commit(
        &self,
        slug: &str,
        commit_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!(
            "/tickets/{slug}/commits/{commit_id}"
        )))
        .await
    }

    pub async fn list_ticket_merge_requests(
        &self,
        slug: &str,
    ) -> Result<MergeRequestListResponse, ApiError> {
        self.get(&self.org_path(&format!("/tickets/{slug}/merge-requests")))
            .await
    }

    pub async fn list_ticket_comments(
        &self,
        slug: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<TicketCommentListResponse, ApiError> {
        let mut path = self.org_path(&format!("/tickets/{slug}/comments"));
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn create_ticket_comment(
        &self,
        slug: &str,
        data: &CreateTicketCommentRequest,
    ) -> Result<TicketComment, ApiError> {
        self.post(
            &self.org_path(&format!("/tickets/{slug}/comments")),
            data,
        )
        .await
    }

    pub async fn update_ticket_comment(
        &self,
        slug: &str,
        comment_id: i64,
        data: &UpdateTicketCommentRequest,
    ) -> Result<TicketComment, ApiError> {
        self.put(
            &self.org_path(&format!("/tickets/{slug}/comments/{comment_id}")),
            data,
        )
        .await
    }

    pub async fn delete_ticket_comment(
        &self,
        slug: &str,
        comment_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!(
            "/tickets/{slug}/comments/{comment_id}"
        )))
        .await
    }
}
