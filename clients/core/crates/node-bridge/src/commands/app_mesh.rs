use napi_derive::napi;

use agentsmesh_types::proto_mesh_state_v1::ReplaceTopologyRequest;
use prost::Message as _;

use crate::AppState;

// Mesh state surface over the shared `runtime.state` (SSOT), mirroring
// app_autopilot.rs / app_loop.rs. Mesh has no realtime events, so this
// fetch-mirror mutator is the only writer feeding `runtime.state.mesh` from
// the TS adapter (after the networking-only fetch_topology returns bytes).
#[napi]
impl AppState {
    #[napi]
    pub fn app_mesh_replace_topology(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceTopologyRequest::decode(&req_bytes[..])
            .map_err(|e| napi::Error::from_reason(format!("decode: {e}")))?;
        if let Some(topology) = req.topology {
            self.runtime.state.write().mesh.set_topology(topology);
        }
        Ok(())
    }
}
