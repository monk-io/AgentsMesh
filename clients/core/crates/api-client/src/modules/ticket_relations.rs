use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_ticket_relations_v1 as tr_proto;

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/ticket_relations/. Procedure paths derive from
// `proto.ticket_relations.v1.TicketRelationsService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn list_relations_connect(
        &self,
        req: &tr_proto::ListRelationsRequest,
    ) -> Result<tr_proto::ListRelationsResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/ListRelations",
            req,
        )
        .await
    }

    pub async fn create_relation_connect(
        &self,
        req: &tr_proto::CreateRelationRequest,
    ) -> Result<tr_proto::Relation, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/CreateRelation",
            req,
        )
        .await
    }

    pub async fn delete_relation_connect(
        &self,
        req: &tr_proto::DeleteRelationRequest,
    ) -> Result<tr_proto::DeleteRelationResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/DeleteRelation",
            req,
        )
        .await
    }

    pub async fn list_ticket_merge_requests_connect(
        &self,
        req: &tr_proto::ListMergeRequestsRequest,
    ) -> Result<tr_proto::ListMergeRequestsResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/ListMergeRequests",
            req,
        )
        .await
    }

    pub async fn list_ticket_commits_connect(
        &self,
        req: &tr_proto::ListCommitsRequest,
    ) -> Result<tr_proto::ListCommitsResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/ListCommits",
            req,
        )
        .await
    }

    pub async fn link_commit_connect(
        &self,
        req: &tr_proto::LinkCommitRequest,
    ) -> Result<tr_proto::Commit, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/LinkCommit",
            req,
        )
        .await
    }

    pub async fn unlink_commit_connect(
        &self,
        req: &tr_proto::UnlinkCommitRequest,
    ) -> Result<tr_proto::UnlinkCommitResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/UnlinkCommit",
            req,
        )
        .await
    }

    pub async fn list_comments_connect(
        &self,
        req: &tr_proto::ListCommentsRequest,
    ) -> Result<tr_proto::ListCommentsResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/ListComments",
            req,
        )
        .await
    }

    pub async fn create_comment_connect(
        &self,
        req: &tr_proto::CreateCommentRequest,
    ) -> Result<tr_proto::Comment, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/CreateComment",
            req,
        )
        .await
    }

    pub async fn update_comment_connect(
        &self,
        req: &tr_proto::UpdateCommentRequest,
    ) -> Result<tr_proto::Comment, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/UpdateComment",
            req,
        )
        .await
    }

    pub async fn delete_comment_connect(
        &self,
        req: &tr_proto::DeleteCommentRequest,
    ) -> Result<tr_proto::DeleteCommentResponse, ApiError> {
        connect_call(
            self,
            "/proto.ticket_relations.v1.TicketRelationsService/DeleteComment",
            req,
        )
        .await
    }
}

// =============================================================================
// Legacy REST methods — preserved for dual-track migration.
// =============================================================================

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
