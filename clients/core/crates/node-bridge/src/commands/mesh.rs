use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    // Networking-only: returns prost-encoded ReplaceTopologyRequest bytes the
    // renderer feeds to getMeshState().replace_topology (runtime.state.mesh SSOT).
    #[napi]
    pub async fn mesh_fetch_topology(&self) -> napi::Result<Buffer> {
        let svc = self.mesh.lock().await;
        let bytes = svc.fetch_topology().await.map_err(err)?;
        Ok(bytes.into())
    }

    // ----- Connect-RPC bridge (binary in / binary out, conventions §2.5) -----

    #[napi]
    pub async fn mesh_get_mesh_topology_connect(&self, request: Buffer) -> napi::Result<Buffer> {
        let svc = self.mesh.lock().await;
        let bytes = svc.get_mesh_topology_connect(request.as_ref()).await.map_err(err)?;
        Ok(bytes.into())
    }

    #[napi]
    pub async fn mesh_get_ticket_pods_connect(&self, request: Buffer) -> napi::Result<Buffer> {
        let svc = self.mesh.lock().await;
        let bytes = svc.get_ticket_pods_connect(request.as_ref()).await.map_err(err)?;
        Ok(bytes.into())
    }

    #[napi]
    pub async fn mesh_batch_get_ticket_pods_connect(&self, request: Buffer) -> napi::Result<Buffer> {
        let svc = self.mesh.lock().await;
        let bytes = svc.batch_get_ticket_pods_connect(request.as_ref()).await.map_err(err)?;
        Ok(bytes.into())
    }

    #[napi]
    pub async fn mesh_create_pod_for_ticket_connect(&self, request: Buffer) -> napi::Result<Buffer> {
        let svc = self.mesh.lock().await;
        let bytes = svc.create_pod_for_ticket_connect(request.as_ref()).await.map_err(err)?;
        Ok(bytes.into())
    }
}
