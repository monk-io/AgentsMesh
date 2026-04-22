use crate::core::AgentsMeshCore;
use crate::dto::{
    ApplyOpsRequestDto, ApplyOpsResultDto, BlockDto, BlockOpDto, BlockRefDto, ChildrenResultDto,
    MeshTopologyDto, NotificationPreferenceListResponseDto, SearchHitDto,
    SemanticSearchRequestDto, SetNotificationPreferenceRequestDto, WorkspaceDto,
};
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    // ── Mesh ──────────────────────────────────────────────

    pub async fn get_mesh_topology(&self) -> Result<MeshTopologyDto, CoreError> {
        let t = self.api.get_mesh_topology().await?;
        Ok(t.into())
    }

    // ── Notifications ─────────────────────────────────────

    pub async fn get_notification_preferences(
        &self,
    ) -> Result<NotificationPreferenceListResponseDto, CoreError> {
        let resp = self.api.get_notification_preferences().await?;
        Ok(resp.into())
    }

    pub async fn set_notification_preference(
        &self,
        req: SetNotificationPreferenceRequestDto,
    ) -> Result<(), CoreError> {
        self.api.set_notification_preference(&req.into()).await?;
        Ok(())
    }

    // ── Blockstore ────────────────────────────────────────

    pub async fn blocks_apply_ops(
        &self,
        req: ApplyOpsRequestDto,
    ) -> Result<ApplyOpsResultDto, CoreError> {
        let resp = self.api.blocks_apply_ops(&req.into()).await?;
        Ok(resp.into())
    }

    pub async fn blocks_list_workspaces(&self) -> Result<Vec<WorkspaceDto>, CoreError> {
        let workspaces = self.api.blocks_list_workspaces().await?;
        Ok(workspaces.into_iter().map(WorkspaceDto::from).collect())
    }

    pub async fn blocks_ensure_default_workspace(&self) -> Result<WorkspaceDto, CoreError> {
        let ws = self.api.blocks_ensure_default_workspace().await?;
        Ok(ws.into())
    }

    pub async fn blocks_get_block(&self, id: String) -> Result<BlockDto, CoreError> {
        let block = self.api.blocks_get_block(&id).await?;
        Ok(block.into())
    }

    pub async fn blocks_list_children(
        &self,
        id: String,
        rel: String,
    ) -> Result<ChildrenResultDto, CoreError> {
        let resp = self.api.blocks_list_children(&id, &rel).await?;
        Ok(resp.into())
    }

    pub async fn blocks_list_backlinks(&self, id: String) -> Result<Vec<BlockRefDto>, CoreError> {
        let refs = self.api.blocks_list_backlinks(&id).await?;
        Ok(refs.into_iter().map(BlockRefDto::from).collect())
    }

    pub async fn blocks_get_subtree(
        &self,
        workspace_id: String,
        root_id: String,
        max_depth: u32,
    ) -> Result<ChildrenResultDto, CoreError> {
        let resp = self
            .api
            .blocks_get_subtree(&workspace_id, &root_id, max_depth)
            .await?;
        Ok(resp.into())
    }

    pub async fn blocks_catchup_ops(
        &self,
        workspace_id: String,
        after: i64,
        limit: u32,
    ) -> Result<Vec<BlockOpDto>, CoreError> {
        let ops = self.api.blocks_catchup_ops(&workspace_id, after, limit).await?;
        Ok(ops.into_iter().map(BlockOpDto::from).collect())
    }

    pub async fn blocks_list_type_defs(
        &self,
        workspace_id: String,
    ) -> Result<Vec<BlockDto>, CoreError> {
        let blocks = self.api.blocks_list_type_defs(&workspace_id).await?;
        Ok(blocks.into_iter().map(BlockDto::from).collect())
    }

    pub async fn blocks_semantic_search(
        &self,
        workspace_id: String,
        req: SemanticSearchRequestDto,
    ) -> Result<Vec<SearchHitDto>, CoreError> {
        let hits = self
            .api
            .blocks_semantic_search(&workspace_id, &req.into())
            .await?;
        Ok(hits.into_iter().map(SearchHitDto::from).collect())
    }
}
