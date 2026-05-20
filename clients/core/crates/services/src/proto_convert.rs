// Proto → legacy serde-shape conversions for the dual-track Connect
// migration. Only the `runner` projector is left — every other domain's
// state crate now stores the proto type directly (R2-S2-{Pod,Mesh,Billing,
// Org,Notification,Message,Repository}). The runner module will follow.
//
// Drop this file together with the runner_state migration.

use agentsmesh_types::proto_runner_api_v1 as runner_proto;
use agentsmesh_types::Runner;

pub mod runner {
    use super::*;

    pub fn from_proto(r: runner_proto::Runner) -> Runner {
        Runner {
            id: r.id,
            // Legacy `name` is best-effort: REST emits the `name` column from
            // backend `runners`. The proto sheds the legacy `name` in favor of
            // `node_id`, so we surface node_id as the display name here — keeps
            // existing state cache consumers (RunnerListItem) rendering.
            name: r.node_id.clone(),
            node_id: option_string(&r.node_id),
            description: option_string(&r.description),
            status: parse_status(&r.status),
            version: r.runner_version,
            max_concurrent_pods: r.max_concurrent_pods,
            active_pod_count: r.current_pods,
            is_enabled: r.is_enabled,
            host_info: if r.host_info_json.is_empty() {
                None
            } else {
                serde_json::from_str(&r.host_info_json).ok()
            },
            last_heartbeat: r.last_heartbeat,
            available_agents: if r.available_agents.is_empty() {
                None
            } else {
                Some(r.available_agents)
            },
            created_at: option_string(&r.created_at),
            updated_at: option_string(&r.updated_at),
        }
    }
}

fn option_string(s: &str) -> Option<String> {
    if s.is_empty() { None } else { Some(s.to_string()) }
}

fn parse_status<T: serde::de::DeserializeOwned + Default>(s: &str) -> T {
    serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_default()
}
