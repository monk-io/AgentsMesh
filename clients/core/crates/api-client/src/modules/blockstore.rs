use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::{
    ApplyOpsRequest, ApplyOpsResult, Block, BlockOp, BlockRef, ChildrenResult,
    SearchHit, SemanticSearchRequest, Workspace,
};
use agentsmesh_types::proto_blockstore_v1 as blockstore_proto;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WorkspacesWrapper {
    #[serde(default)]
    workspaces: Vec<Workspace>,
}

#[derive(Debug, Deserialize)]
struct OpsWrapper {
    #[serde(default)]
    ops: Vec<BlockOp>,
}

#[derive(Debug, Deserialize)]
struct BlocksWrapper {
    #[serde(default)]
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize)]
struct BacklinksWrapper {
    #[serde(default)]
    refs: Vec<BlockRef>,
}

#[derive(Debug, Deserialize)]
struct HitsWrapper {
    #[serde(default)]
    hits: Vec<SearchHit>,
}

impl ApiClient {
    pub async fn blocks_apply_ops(&self, req: &ApplyOpsRequest) -> Result<ApplyOpsResult, ApiError> {
        self.post(&self.org_path("/blocks/ops"), req).await
    }

    pub async fn blocks_list_workspaces(&self) -> Result<Vec<Workspace>, ApiError> {
        let w: WorkspacesWrapper = self.get(&self.org_path("/blocks/workspaces")).await?;
        Ok(w.workspaces)
    }

    pub async fn blocks_ensure_default_workspace(&self) -> Result<Workspace, ApiError> {
        self.post(&self.org_path("/blocks/workspaces/default"), &serde_json::json!({})).await
    }

    pub async fn blocks_get_block(&self, id: &str) -> Result<Block, ApiError> {
        let enc = urlencoding::encode(id);
        self.get(&self.org_path(&format!("/blocks/{enc}"))).await
    }

    pub async fn blocks_list_children(&self, id: &str, rel: &str) -> Result<ChildrenResult, ApiError> {
        let enc = urlencoding::encode(id);
        let relenc = urlencoding::encode(rel);
        self.get(&self.org_path(&format!("/blocks/{enc}/children?rel={relenc}"))).await
    }

    pub async fn blocks_list_backlinks(&self, id: &str) -> Result<Vec<BlockRef>, ApiError> {
        let enc = urlencoding::encode(id);
        let w: BacklinksWrapper = self.get(&self.org_path(&format!("/blocks/{enc}/backlinks"))).await?;
        Ok(w.refs)
    }

    pub async fn blocks_get_subtree(
        &self, workspace_id: &str, root_id: &str, max_depth: u32,
    ) -> Result<ChildrenResult, ApiError> {
        let ws = urlencoding::encode(workspace_id);
        let root = urlencoding::encode(root_id);
        self.get(&self.org_path(&format!(
            "/blocks/workspaces/{ws}/subtree?root={root}&max_depth={max_depth}",
        ))).await
    }

    pub async fn blocks_catchup_ops(
        &self, workspace_id: &str, after: i64, limit: u32,
    ) -> Result<Vec<BlockOp>, ApiError> {
        let ws = urlencoding::encode(workspace_id);
        let w: OpsWrapper = self.get(&self.org_path(&format!(
            "/blocks/workspaces/{ws}/ops?after={after}&limit={limit}",
        ))).await?;
        Ok(w.ops)
    }

    pub async fn blocks_list_type_defs(&self, workspace_id: &str) -> Result<Vec<Block>, ApiError> {
        let ws = urlencoding::encode(workspace_id);
        let w: BlocksWrapper = self.get(&self.org_path(&format!("/blocks/workspaces/{ws}/type-defs"))).await?;
        Ok(w.blocks)
    }

    pub async fn blocks_semantic_search(
        &self, workspace_id: &str, req: &SemanticSearchRequest,
    ) -> Result<Vec<SearchHit>, ApiError> {
        let ws = urlencoding::encode(workspace_id);
        let w: HitsWrapper = self.post(&self.org_path(&format!("/blocks/workspaces/{ws}/search")), req).await?;
        Ok(w.hits)
    }
}

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/blockstore/. Procedure paths derive from
// `proto.blockstore.v1.BlockstoreService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn blockstore_apply_ops_connect(
        &self,
        req: &blockstore_proto::ApplyOpsRequest,
    ) -> Result<blockstore_proto::ApplyOpsResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ApplyOps", req).await
    }

    pub async fn blockstore_list_workspaces_connect(
        &self,
        req: &blockstore_proto::ListWorkspacesRequest,
    ) -> Result<blockstore_proto::ListWorkspacesResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ListWorkspaces", req).await
    }

    pub async fn blockstore_ensure_default_workspace_connect(
        &self,
        req: &blockstore_proto::EnsureDefaultWorkspaceRequest,
    ) -> Result<blockstore_proto::Workspace, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/EnsureDefaultWorkspace", req).await
    }

    pub async fn blockstore_create_workspace_connect(
        &self,
        req: &blockstore_proto::CreateWorkspaceRequest,
    ) -> Result<blockstore_proto::Workspace, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/CreateWorkspace", req).await
    }

    pub async fn blockstore_delete_workspace_connect(
        &self,
        req: &blockstore_proto::DeleteWorkspaceRequest,
    ) -> Result<blockstore_proto::DeleteWorkspaceResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/DeleteWorkspace", req).await
    }

    pub async fn blockstore_get_block_connect(
        &self,
        req: &blockstore_proto::GetBlockRequest,
    ) -> Result<blockstore_proto::Block, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/GetBlock", req).await
    }

    pub async fn blockstore_list_children_connect(
        &self,
        req: &blockstore_proto::ListChildrenRequest,
    ) -> Result<blockstore_proto::ChildrenResult, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ListChildren", req).await
    }

    pub async fn blockstore_list_backlinks_connect(
        &self,
        req: &blockstore_proto::ListBacklinksRequest,
    ) -> Result<blockstore_proto::ListBacklinksResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ListBacklinks", req).await
    }

    pub async fn blockstore_get_subtree_connect(
        &self,
        req: &blockstore_proto::GetSubtreeRequest,
    ) -> Result<blockstore_proto::ChildrenResult, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/GetSubtree", req).await
    }

    pub async fn blockstore_stream_ops_connect(
        &self,
        req: &blockstore_proto::StreamOpsRequest,
    ) -> Result<blockstore_proto::StreamOpsResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/StreamOps", req).await
    }

    pub async fn blockstore_export_workspace_connect(
        &self,
        req: &blockstore_proto::ExportWorkspaceRequest,
    ) -> Result<blockstore_proto::ExportWorkspaceResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ExportWorkspace", req).await
    }

    pub async fn blockstore_list_type_defs_connect(
        &self,
        req: &blockstore_proto::ListTypeDefsRequest,
    ) -> Result<blockstore_proto::ListTypeDefsResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/ListTypeDefs", req).await
    }

    pub async fn blockstore_get_block_at_connect(
        &self,
        req: &blockstore_proto::GetBlockAtRequest,
    ) -> Result<blockstore_proto::Block, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/GetBlockAt", req).await
    }

    pub async fn blockstore_semantic_search_connect(
        &self,
        req: &blockstore_proto::SemanticSearchRequest,
    ) -> Result<blockstore_proto::SemanticSearchResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/SemanticSearch", req).await
    }

    pub async fn blockstore_memory_retrieve_connect(
        &self,
        req: &blockstore_proto::MemoryRetrieveRequest,
    ) -> Result<blockstore_proto::MemoryRetrieveResponse, ApiError> {
        connect_call(self, "/proto.blockstore.v1.BlockstoreService/MemoryRetrieve", req).await
    }
}
