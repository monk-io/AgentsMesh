use crate::mesh_state::MeshState;
use agentsmesh_types::*;

fn sample_topology() -> MeshTopology {
    MeshTopology {
        nodes: vec![
            MeshNode { pod_key: "p1".into(), alias: Some("worker".into()), status: PodStatus::Running, agent_status: Some("idle".into()), agent_slug: "claude".into(), runner_id: Some(1), ..Default::default() },
            MeshNode { pod_key: "p2".into(), status: PodStatus::Creating, agent_slug: "aider".into(), runner_id: Some(1), ..Default::default() },
            MeshNode { pod_key: "p3".into(), status: PodStatus::Terminated, agent_slug: "claude".into(), runner_id: Some(2), ..Default::default() },
        ],
        edges: vec![MeshEdge { source: "p1".into(), target: "p2".into(), binding_status: Some("bound".into()), ..Default::default() }],
        channels: vec![MeshChannelInfo { id: 1, name: "general".into(), pod_keys: vec!["p1".into(), "p2".into()], ..Default::default() }],
        runners: vec![
            MeshRunnerInfo { id: 1, name: "r1".into(), status: RunnerStatus::Online, pod_keys: vec!["p1".into(), "p2".into()], ..Default::default() },
            MeshRunnerInfo { id: 2, name: "r2".into(), status: RunnerStatus::Offline, pod_keys: vec!["p3".into()], ..Default::default() },
        ],
    }
}

#[test] fn new_is_empty() { let s = MeshState::new(); assert!(s.topology().is_none()); assert!(s.selected_node().is_none()); }
#[test] fn set_topology() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.topology().unwrap().nodes.len(), 3); }
#[test] fn clear_topology() { let mut s = MeshState::new(); s.set_topology(sample_topology()); s.clear_topology(); assert!(s.topology().is_none()); }
#[test] fn select_node() { let mut s = MeshState::new(); s.select_node(Some("p1".into())); assert_eq!(s.selected_node(), Some("p1")); s.select_node(None); assert!(s.selected_node().is_none()); }
#[test] fn get_node_by_key() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_node_by_key("p1").unwrap().agent_slug, "claude"); assert!(s.get_node_by_key("x").is_none()); }
#[test] fn get_node_no_topo() { let s = MeshState::new(); assert!(s.get_node_by_key("p1").is_none()); }
#[test] fn get_edges() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_edges_for_node("p1").len(), 1); assert!(s.get_edges_for_node("p3").is_empty()); }
#[test] fn get_edges_no_topo() { let s = MeshState::new(); assert!(s.get_edges_for_node("p1").is_empty()); }
#[test] fn get_channels() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_channels_for_node("p1").len(), 1); assert!(s.get_channels_for_node("p3").is_empty()); }
#[test] fn get_channels_no_topo() { let s = MeshState::new(); assert!(s.get_channels_for_node("p1").is_empty()); }
#[test] fn get_nodes_by_runner() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_nodes_by_runner(1).len(), 2); assert!(s.get_nodes_by_runner(99).is_empty()); }
#[test] fn get_active_nodes() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_active_nodes().len(), 2); }
#[test] fn get_runner_info() { let mut s = MeshState::new(); s.set_topology(sample_topology()); assert_eq!(s.get_runner_info(1).unwrap().name, "r1"); assert!(s.get_runner_info(99).is_none()); }
