use crate::core::AgentsMeshCore;
use crate::dto::{
    BlockDto, ChildrenResultDto, MeshTopologyDto,
    NotificationPreferenceListResponseDto, SearchHitDto,
    SemanticSearchRequestDto, SetNotificationPreferenceRequestDto, WorkspaceDto,
};
use crate::error::CoreError;

#[uniffi::export(async_runtime = "tokio")]
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

    // ── Blockstore ─ all routes the local SSOT (`self.blockstore`) ──
    // Mutations land in service state; sync flat-map readers below
    // serve from the same state without a backend round-trip — same
    // contract the WebView-embedded `BlockstoreService` expects.

    pub async fn blocks_apply_ops(&self, req_json: String) -> Result<String, CoreError> {
        self.blockstore.apply_ops(&req_json).await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_list_workspaces_json(&self) -> Result<String, CoreError> {
        self.blockstore.list_workspaces().await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_ensure_default_workspace_json(&self) -> Result<String, CoreError> {
        self.blockstore.ensure_default_workspace().await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_load_subtree(
        &self, workspace_id: String, root_id: String,
    ) -> Result<(), CoreError> {
        self.blockstore.load_subtree(&workspace_id, &root_id).await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_load_type_defs(&self, workspace_id: String) -> Result<(), CoreError> {
        self.blockstore.load_type_defs(&workspace_id).await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_catchup(&self, workspace_id: String) -> Result<(), CoreError> {
        self.blockstore.catchup(&workspace_id).await.map_err(|m| CoreError::Unknown { message: m })
    }

    pub async fn blocks_semantic_search_json(
        &self, workspace_id: String, req_json: String,
    ) -> Result<String, CoreError> {
        self.blockstore
            .semantic_search(&workspace_id, &req_json).await
            .map_err(|m| CoreError::Unknown { message: m })
    }

    pub fn blocks_apply_remote_op(&self, op_json: String) -> Result<(), CoreError> {
        self.blockstore.apply_remote_op(&op_json).map_err(|m| CoreError::Unknown { message: m })
    }

    // Sync flat-map readers — return string snapshots of the in-process
    // state. WebView's RpcBlockstoreService caches these and serves
    // synchronous reads (`blocks_json()` etc.) from cache.
    pub fn blocks_workspaces_json(&self) -> String { self.blockstore.workspaces_json() }
    pub fn blocks_blocks_json(&self) -> String { self.blockstore.blocks_json() }
    pub fn blocks_refs_json(&self) -> String { self.blockstore.refs_json() }
    pub fn blocks_nest_children_json(&self) -> String { self.blockstore.nest_children_json() }
    pub fn blocks_backlinks_json(&self) -> String { self.blockstore.backlinks_json() }
    pub fn blocks_last_op_ids_json(&self) -> String { self.blockstore.last_op_ids_json() }

    pub fn blocks_get_block_json(&self, id: String) -> Option<String> {
        self.blockstore.get_block_json(&id)
    }
    pub fn blocks_list_children_json(&self, parent_id: String) -> String {
        self.blockstore.list_children_json(&parent_id)
    }
    pub fn blocks_list_backlinks_json(&self, target_id: String) -> String {
        self.blockstore.list_backlinks_json(&target_id)
    }
    pub fn blocks_type_defs_json(&self, workspace_id: String) -> String {
        self.blockstore.type_defs_json(&workspace_id)
    }
    pub fn blocks_last_op_id(&self, workspace_id: String) -> i64 {
        self.blockstore.last_op_id(&workspace_id)
    }
    pub fn blocks_set_last_op_id(&self, workspace_id: String, id: i64) {
        self.blockstore.set_last_op_id(&workspace_id, id);
    }

    /// Search-hit DTO export — kept typed so a Swift caller (e.g. a
    /// future native semantic-search UI) doesn't have to JSON-parse.
    pub async fn blocks_semantic_search(
        &self, workspace_id: String, req: SemanticSearchRequestDto,
    ) -> Result<Vec<SearchHitDto>, CoreError> {
        let req: agentsmesh_types::SemanticSearchRequest = req.into();
        let req_json = serde_json::to_string(&req).unwrap_or_else(|_| "{}".into());
        let resp = self.blockstore
            .semantic_search(&workspace_id, &req_json).await
            .map_err(|m| CoreError::Unknown { message: m })?;
        let hits: Vec<agentsmesh_types::SearchHit> = serde_json::from_str(&resp)
            .map_err(|e| CoreError::Unknown { message: format!("decode hits: {e}") })?;
        Ok(hits.into_iter().map(SearchHitDto::from).collect())
    }

    // ── Typed wrappers — for native TCA reducers that prefer Swift
    // structs over JSON strings. Each one fans out the JSON call and
    // decodes once on the Rust side; saves 2 JSON parses on Swift side.

    pub async fn blocks_ensure_default_workspace(&self) -> Result<WorkspaceDto, CoreError> {
        let json = self.blockstore.ensure_default_workspace().await
            .map_err(|m| CoreError::Unknown { message: m })?;
        let ws: agentsmesh_types::Workspace = serde_json::from_str(&json)
            .map_err(|e| CoreError::Unknown { message: format!("decode workspace: {e}") })?;
        Ok(ws.into())
    }

    pub async fn blocks_list_workspaces(&self) -> Result<Vec<WorkspaceDto>, CoreError> {
        let json = self.blockstore.list_workspaces().await.map_err(|m| CoreError::Unknown { message: m })?;
        #[derive(serde::Deserialize)]
        struct Resp { workspaces: Vec<agentsmesh_types::Workspace> }
        let r: Resp = serde_json::from_str(&json)
            .map_err(|e| CoreError::Unknown { message: format!("decode workspaces: {e}") })?;
        Ok(r.workspaces.into_iter().map(WorkspaceDto::from).collect())
    }

    pub async fn blocks_get_block(&self, id: String) -> Result<BlockDto, CoreError> {
        let json = self.blockstore.get_block_json(&id)
            .ok_or_else(|| CoreError::Unknown { message: format!("block not found: {id}") })?;
        let b: agentsmesh_types::Block = serde_json::from_str(&json)
            .map_err(|e| CoreError::Unknown { message: format!("decode block: {e}") })?;
        Ok(b.into())
    }

    pub async fn blocks_get_subtree(
        &self, workspace_id: String, root_id: String, _max_depth: u32,
    ) -> Result<ChildrenResultDto, CoreError> {
        self.blockstore.load_subtree(&workspace_id, &root_id).await
            .map_err(|m| CoreError::Unknown { message: m })?;
        let children_json = self.blockstore.list_children_json(&root_id);
        let children: agentsmesh_types::ChildrenResult = serde_json::from_str(&children_json)
            .unwrap_or(agentsmesh_types::ChildrenResult { blocks: vec![], refs: vec![] });
        Ok(children.into())
    }
}
