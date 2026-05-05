use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn get_mesh_topology(&self) -> Result<MeshTopology, ApiError> {
        self.get_resource(&self.org_path("/mesh/topology"), "topology").await
    }
}
