use serde::{Deserialize, Serialize};

use crate::{PodStatus, RunnerStatus};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeshTopology {
    pub nodes: Vec<MeshNode>,
    pub edges: Vec<MeshEdge>,
    pub channels: Vec<MeshChannelInfo>,
    pub runners: Vec<MeshRunnerInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeshNode {
    pub pod_key: String,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub status: PodStatus,
    #[serde(default)]
    pub agent_status: Option<String>,
    #[serde(default)]
    pub agent_slug: String,
    #[serde(default)]
    pub runner_id: Option<i64>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub ticket_id: Option<i64>,
    #[serde(default)]
    pub ticket_slug: Option<String>,
    #[serde(default)]
    pub ticket_title: Option<String>,
    #[serde(default)]
    pub repository_id: Option<i64>,
    #[serde(default)]
    pub created_by_id: Option<i64>,
    #[serde(default)]
    pub runner_node_id: Option<String>,
    #[serde(default)]
    pub runner_status: Option<String>,
    #[serde(default)]
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeshEdge {
    #[serde(default)]
    pub id: Option<i64>,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub binding_status: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub granted_scopes: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeshChannelInfo {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub pod_keys: Vec<String>,
    #[serde(default)]
    pub message_count: Option<i64>,
    #[serde(default)]
    pub is_archived: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeshRunnerInfo {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: RunnerStatus,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub max_concurrent_pods: Option<i32>,
    #[serde(default)]
    pub current_pods: Option<i32>,
    #[serde(default)]
    pub pod_keys: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn mesh_topology_roundtrip() {
        let topo = MeshTopology {
            nodes: vec![MeshNode {
                pod_key: "pod-1".into(),
                alias: Some("worker".into()),
                status: PodStatus::Running,
                agent_status: Some("active".into()),
                agent_slug: "claude".into(),
                runner_id: Some(1),
                ..Default::default()
            }],
            edges: vec![MeshEdge {
                source: "pod-1".into(),
                target: "pod-2".into(),
                binding_status: Some("bound".into()),
                ..Default::default()
            }],
            channels: vec![MeshChannelInfo {
                id: 1,
                name: "general".into(),
                pod_keys: vec!["pod-1".into(), "pod-2".into()],
                ..Default::default()
            }],
            runners: vec![MeshRunnerInfo {
                id: 1,
                name: "runner-1".into(),
                status: RunnerStatus::Online,
                pod_keys: vec!["pod-1".into()],
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&topo).unwrap();
        let decoded: MeshTopology = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.nodes.len(), 1);
        assert_eq!(decoded.edges.len(), 1);
        assert_eq!(decoded.channels[0].pod_keys.len(), 2);
    }

    #[test]
    fn mesh_topology_empty() {
        let json = r#"{"nodes":[],"edges":[],"channels":[],"runners":[]}"#;
        let topo: MeshTopology = serde_json::from_str(json).unwrap();
        assert!(topo.nodes.is_empty());
        assert!(topo.edges.is_empty());
    }

    #[test]
    fn mesh_node_minimal() {
        let json = r#"{"pod_key":"p","status":"s","agent_slug":"a"}"#;
        let node: MeshNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.pod_key, "p");
        assert!(node.alias.is_none());
        assert!(node.runner_id.is_none());
    }

    #[test]
    fn mesh_edge_minimal() {
        let json = r#"{"source":"a","target":"b"}"#;
        let edge: MeshEdge = serde_json::from_str(json).unwrap();
        assert_eq!(edge.source, "a");
        assert!(edge.binding_status.is_none());
    }

    #[test]
    fn mesh_channel_info_empty_pods() {
        let json = r#"{"id":1,"name":"ch","pod_keys":[]}"#;
        let ch: MeshChannelInfo = serde_json::from_str(json).unwrap();
        assert!(ch.pod_keys.is_empty());
    }

    #[test]
    fn mesh_runner_info_roundtrip() {
        let info = MeshRunnerInfo {
            id: 1,
            name: "r".into(),
            status: RunnerStatus::Online,
            pod_keys: vec!["p1".into(), "p2".into()],
            ..Default::default()
        };
        let json = serde_json::to_string(&info).unwrap();
        let decoded: MeshRunnerInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.pod_keys.len(), 2);
    }
}
