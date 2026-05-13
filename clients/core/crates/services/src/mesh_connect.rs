// Connect-RPC bridge methods for MeshService. Binary in, binary out
// (conventions §2.5). Each method:
//   1. Decodes the prost-encoded request bytes from the wasm bridge.
//   2. Forwards to the api-client `*_connect` method.
//   3. Encodes the response back to prost bytes.

use agentsmesh_types::proto_mesh_v1 as mp;
use prost::Message;

use crate::mesh::MeshService;
use crate::wire;

macro_rules! connect_bridge {
    ($name:ident, $req:ident, $client_call:ident) => {
        pub async fn $name(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
            let req = mp::$req::decode(request_bytes)
                .map_err(|e| format!("decode {}: {e}", stringify!($req)))?;
            let resp = self.client_ref().$client_call(&req).await.map_err(wire)?;
            Ok(resp.encode_to_vec())
        }
    };
}

impl MeshService {
    connect_bridge!(
        get_mesh_topology_connect,
        GetMeshTopologyRequest,
        get_mesh_topology_connect
    );
    connect_bridge!(
        get_ticket_pods_connect,
        GetTicketPodsRequest,
        get_ticket_pods_connect
    );
    connect_bridge!(
        batch_get_ticket_pods_connect,
        BatchGetTicketPodsRequest,
        batch_get_ticket_pods_connect
    );
    connect_bridge!(
        create_pod_for_ticket_connect,
        CreatePodForTicketRequest,
        create_pod_for_ticket_connect
    );
}
