// Connect-RPC bridge methods for BlockstoreService. Binary in, binary out
// (conventions §2.5). Each method:
//   1. Decodes the prost-encoded request bytes from the wasm bridge.
//   2. Forwards to the api-client `*_connect` method (which speaks
//      application/proto to the Connect handler).
//   3. Encodes the response back to prost bytes for the bridge to return.
//
// Block.data / .meta / BlockOp.payload / .forward / .inverse / .context are
// opaque UTF-8 JSON strings on the wire — the local state cache (blockstore.rs)
// keeps the existing `serde_json::Value` shape and is unchanged. Bridging
// from the proto layer back into the cache (so Connect-path applies populate
// state) lands after every client is on Connect; until then the legacy
// `apply_ops` JSON method continues to maintain cache invariants.

use agentsmesh_types::proto_blockstore_v1 as blockstore_proto;
use prost::Message;

use crate::blockstore::BlockstoreService;
use crate::wire;

macro_rules! connect_bridge {
    ($name:ident, $req:ident, $client_call:ident) => {
        pub async fn $name(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
            let req = blockstore_proto::$req::decode(request_bytes)
                .map_err(|e| format!("decode {}: {e}", stringify!($req)))?;
            let resp = self.client().$client_call(&req).await.map_err(wire)?;
            Ok(resp.encode_to_vec())
        }
    };
}

impl BlockstoreService {
    connect_bridge!(apply_ops_connect, ApplyOpsRequest, blockstore_apply_ops_connect);
    connect_bridge!(list_workspaces_connect, ListWorkspacesRequest, blockstore_list_workspaces_connect);
    connect_bridge!(ensure_default_workspace_connect, EnsureDefaultWorkspaceRequest, blockstore_ensure_default_workspace_connect);
    connect_bridge!(create_workspace_connect, CreateWorkspaceRequest, blockstore_create_workspace_connect);
    connect_bridge!(delete_workspace_connect, DeleteWorkspaceRequest, blockstore_delete_workspace_connect);
    connect_bridge!(get_block_connect, GetBlockRequest, blockstore_get_block_connect);
    connect_bridge!(list_children_connect, ListChildrenRequest, blockstore_list_children_connect);
    connect_bridge!(list_backlinks_connect, ListBacklinksRequest, blockstore_list_backlinks_connect);
    connect_bridge!(get_subtree_connect, GetSubtreeRequest, blockstore_get_subtree_connect);
    connect_bridge!(stream_ops_connect, StreamOpsRequest, blockstore_stream_ops_connect);
    connect_bridge!(export_workspace_connect, ExportWorkspaceRequest, blockstore_export_workspace_connect);
    connect_bridge!(list_type_defs_connect, ListTypeDefsRequest, blockstore_list_type_defs_connect);
    connect_bridge!(get_block_at_connect, GetBlockAtRequest, blockstore_get_block_at_connect);
    connect_bridge!(semantic_search_connect, SemanticSearchRequest, blockstore_semantic_search_connect);
    connect_bridge!(memory_retrieve_connect, MemoryRetrieveRequest, blockstore_memory_retrieve_connect);
}
