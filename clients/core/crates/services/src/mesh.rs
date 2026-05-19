use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::mesh_state::MeshState;
use agentsmesh_types::MeshTopology;

pub struct MeshService {
    client: Arc<ApiClient>,
    state: RwLock<MeshState>,
}

impl MeshService {
    pub fn new(client: Arc<ApiClient>, state: MeshState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    /// Crate-local accessor used by mesh_connect.rs to forward to the
    /// underlying api-client `*_connect` methods.
    pub(crate) fn client_ref(&self) -> &ApiClient {
        &self.client
    }

    pub fn topology_json(&self) -> Option<String> {
        self.state.read().unwrap().topology()
            .map(|t| serde_json::to_string(t).unwrap_or_default())
    }

    pub fn selected_node(&self) -> Option<String> {
        self.state.read().unwrap().selected_node().map(|s| s.to_string())
    }

    pub fn get_node_json(&self, pod_key: &str) -> Option<String> {
        self.state.read().unwrap().get_node_by_key(pod_key)
            .map(|n| serde_json::to_string(n).unwrap_or_default())
    }

    pub fn get_edges_for_node_json(&self, pod_key: &str) -> String {
        let binding = self.state.read().unwrap();
        let edges = binding.get_edges_for_node(pod_key);
        serde_json::to_string(&edges).unwrap_or_default()
    }

    pub fn get_channels_for_node_json(&self, pod_key: &str) -> String {
        let binding = self.state.read().unwrap();
        let chs = binding.get_channels_for_node(pod_key);
        serde_json::to_string(&chs).unwrap_or_default()
    }

    pub fn get_active_nodes_json(&self) -> String {
        let binding = self.state.read().unwrap();
        let nodes = binding.get_active_nodes();
        serde_json::to_string(&nodes).unwrap_or_default()
    }

    pub fn get_nodes_by_runner_json(&self, runner_id: i64) -> String {
        let binding = self.state.read().unwrap();
        let nodes = binding.get_nodes_by_runner(runner_id);
        serde_json::to_string(&nodes).unwrap_or_default()
    }

    pub fn get_runner_info_json(&self, runner_id: i64) -> Option<String> {
        self.state.read().unwrap().get_runner_info(runner_id)
            .map(|r| serde_json::to_string(r).unwrap_or_default())
    }

    pub fn clear_topology(&self) {
        self.state.write().unwrap().clear_topology();
    }

    pub fn select_node(&self, pod_key: Option<String>) {
        self.state.write().unwrap().select_node(pod_key);
    }

    pub async fn fetch_topology(&self) -> Result<String, String> {
        use agentsmesh_types::proto_mesh_v1 as mp;
        let req = mp::GetMeshTopologyRequest {
            org_slug: self.client.current_org_slug(),
        };
        let proto_topo = self.client
            .get_mesh_topology_connect(&req)
            .await.map_err(crate::wire)?;
        let topo = mesh_proto_to_legacy(&proto_topo);
        self.state.write().unwrap().set_topology(topo.clone());
        serde_json::to_string(&topo).map_err(crate::wire)
    }
}

// mesh_proto_to_legacy converts the prost wire shape into the legacy serde
// `MeshTopology` so the renderer's state slots can continue reading the
// existing field set. Drop alongside the legacy serde types once every
// renderer call site reads the proto camelCase shape directly.
fn mesh_proto_to_legacy(p: &agentsmesh_types::proto_mesh_v1::MeshTopology) -> MeshTopology {
    use agentsmesh_types::{MeshChannelInfo, MeshEdge, MeshNode, MeshRunnerInfo, PodStatus};
    fn parse_status(s: &str) -> PodStatus {
        serde_json::from_value::<PodStatus>(serde_json::Value::String(s.to_string()))
            .unwrap_or_default()
    }
    fn parse_runner_status(s: &str) -> agentsmesh_types::RunnerStatus {
        serde_json::from_value::<agentsmesh_types::RunnerStatus>(serde_json::Value::String(s.to_string()))
            .unwrap_or_default()
    }
    MeshTopology {
        nodes: p.nodes.iter().map(|n| MeshNode {
            pod_key: n.pod_key.clone(),
            alias: n.alias.clone(),
            status: parse_status(&n.status),
            agent_status: if n.agent_status.is_empty() { None } else { Some(n.agent_status.clone()) },
            agent_slug: n.agent_slug.clone(),
            runner_id: if n.runner_id == 0 { None } else { Some(n.runner_id) },
            model: n.model.clone(),
            title: n.title.clone(),
            ticket_id: n.ticket_id,
            ticket_slug: n.ticket_slug.clone(),
            ticket_title: n.ticket_title.clone(),
            repository_id: n.repository_id,
            created_by_id: if n.created_by_id == 0 { None } else { Some(n.created_by_id) },
            runner_node_id: if n.runner_node_id.is_empty() { None } else { Some(n.runner_node_id.clone()) },
            runner_status: if n.runner_status.is_empty() { None } else { Some(n.runner_status.clone()) },
            started_at: n.started_at.clone(),
        }).collect(),
        edges: p.edges.iter().map(|e| MeshEdge {
            id: if e.id == 0 { None } else { Some(e.id) },
            source: e.source.clone(),
            target: e.target.clone(),
            binding_status: if e.status.is_empty() { None } else { Some(e.status.clone()) },
            status: if e.status.is_empty() { None } else { Some(e.status.clone()) },
            granted_scopes: if e.granted_scopes.is_empty() { None } else { Some(e.granted_scopes.clone()) },
            pending_scopes: if e.pending_scopes.is_empty() { None } else { Some(e.pending_scopes.clone()) },
        }).collect(),
        channels: p.channels.iter().map(|c| MeshChannelInfo {
            id: c.id,
            name: c.name.clone(),
            description: c.description.clone(),
            pod_keys: c.pod_keys.clone(),
            message_count: Some(c.message_count as i64),
            is_archived: Some(c.is_archived),
        }).collect(),
        runners: p.runners.iter().map(|r| MeshRunnerInfo {
            id: r.id,
            name: r.node_id.clone(),
            status: parse_runner_status(&r.status),
            node_id: if r.node_id.is_empty() { None } else { Some(r.node_id.clone()) },
            max_concurrent_pods: Some(r.max_concurrent_pods),
            current_pods: Some(r.current_pods),
            pod_keys: Vec::new(),
        }).collect(),
    }
}
