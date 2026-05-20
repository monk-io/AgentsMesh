// Proto → legacy serde-shape conversions for the dual-track Connect
// migration. Service public method signatures (e.g., `fetch_runners`,
// `fetch_repositories`) preserve the legacy JSON wire shape so wasm bridge /
// iOS FFI / node-bridge consumers don't change. The internal implementation
// calls Connect-RPC and routes the prost response through these converters
// back to the legacy shape.
//
// R2 progress: the `pod` module has been retired — PodState cache now stores
// proto.pod.v1.Pod directly (see clients/core/crates/state/src/pod_state.rs).
// Remaining modules (runner / repository) get retired as their respective
// state crates migrate. The ticket module is already empty.

use agentsmesh_types::proto_repository_v1 as repo_proto;
use agentsmesh_types::proto_runner_api_v1 as runner_proto;
use agentsmesh_types::{Repository, Runner};

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
            node_id: super::option_string(&r.node_id),
            description: super::option_string(&r.description),
            status: super::parse_status(&r.status),
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
            created_at: super::option_string(&r.created_at),
            updated_at: super::option_string(&r.updated_at),
        }
    }
}

pub mod repository {
    use super::*;

    pub fn from_proto(r: repo_proto::Repository) -> Repository {
        Repository {
            id: r.id,
            name: r.name,
            slug: super::option_string(&r.slug),
            provider_type: super::option_string(&r.provider_type),
            provider_base_url: super::option_string(&r.provider_base_url),
            http_clone_url: super::option_string(&r.http_clone_url),
            ssh_clone_url: super::option_string(&r.ssh_clone_url),
            external_id: super::option_string(&r.external_id),
            default_branch: super::option_string(&r.default_branch),
            ticket_prefix: r.ticket_prefix,
            visibility: super::option_string(&r.visibility),
            is_active: Some(r.is_active),
            created_at: super::option_string(&r.created_at),
            updated_at: super::option_string(&r.updated_at),
        }
    }
}

pub mod ticket {}

fn option_string(s: &str) -> Option<String> {
    if s.is_empty() { None } else { Some(s.to_string()) }
}

fn parse_status<T: serde::de::DeserializeOwned + Default>(s: &str) -> T {
    serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_default()
}
