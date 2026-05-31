use napi_derive::napi;

use agentsmesh_types::proto_channel_state_v1::{
    RemoveChannelMemberRequest, ReplaceChannelMembersRequest, ReplaceChannelPodsRequest,
};
use prost::Message as _;

use crate::AppState;

// Channel pods/members surface over the shared `runtime.state` (the
// dispatch-hook SSOT) — the migrated counterpart to the legacy
// `channel_replace_channel_pods` / `channel_channel_pods_json` commands that
// targeted the disjoint per-service ChannelService cache. Keeping pods/members
// on `runtime.state` lets the renderer's snapshot mirror reflect them together
// with realtime + fetched baseline (the same rationale as app_channel.rs).
fn decode_err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(format!("decode: {e}"))
}

#[napi]
impl AppState {
    #[napi]
    pub fn app_channel_pods_json(&self, channel_id: i64) -> String {
        serde_json::to_string(&self.runtime.state.read().channels.get_channel_pods(channel_id))
            .unwrap_or_else(|_| "[]".to_string())
    }

    #[napi]
    pub fn app_channel_replace_pods(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceChannelPodsRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().channels.set_channel_pods(req.channel_id, req.pods);
        Ok(())
    }

    #[napi]
    pub fn app_channel_replace_members(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = ReplaceChannelMembersRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().channels.set_channel_members(req.channel_id, req.members);
        Ok(())
    }

    #[napi]
    pub fn app_channel_remove_member(&self, req_bytes: Vec<u8>) -> napi::Result<()> {
        let req = RemoveChannelMemberRequest::decode(&req_bytes[..]).map_err(decode_err)?;
        self.runtime.state.write().channels.remove_channel_member(req.channel_id, req.user_id);
        Ok(())
    }
}
