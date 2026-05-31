use std::sync::Arc;

use agentsmesh_state::app_state::AppState;
use agentsmesh_types::proto_ticket_state_v1::{
    ApplyTicketDeletedEventRequest, ApplyTicketStatusEventRequest,
    AppendBoardColumnTicketsRequest, FilterTicketsRequest, FilterTicketsResponse,
    InsertCreatedLabelRequest, InsertCreatedTicketRequest,
    PatchCachedTicketRequest, RemoveCachedLabelRequest,
    ReplaceBoardColumnsRequest, ReplaceCachedLabelsRequest,
    ReplaceCachedTicketsRequest, SetCurrentTicketRequest,
};
use parking_lot::RwLock;
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTicketState {
    state: Arc<RwLock<AppState>>,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

impl WasmTicketState {
    pub(crate) fn from_runtime(state: Arc<RwLock<AppState>>) -> Self {
        Self { state }
    }
}

#[wasm_bindgen]
impl WasmTicketState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::with_storage(crate::new_memory_backend()))),
        }
    }

    pub fn tickets_json(&self) -> String {
        serde_json::to_string(self.state.read().tickets.get_tickets()).unwrap_or_default()
    }

    // ticket→pods cache moved off the orphan TicketService onto runtime.state
    // (the dispatch-hook SSOT). `useTicketPods` fetches via the service then
    // mirrors the result here for synchronous React reads.
    pub fn ticket_pods_json(&self, slug: &str) -> String {
        serde_json::to_string(&self.state.read().tickets.get_ticket_pods(slug))
            .unwrap_or_else(|_| "[]".to_string())
    }

    pub fn set_ticket_pods(&self, slug: &str, pods_json: &str) -> Result<(), JsValue> {
        let pods: Vec<agentsmesh_types::proto_pod_v1::Pod> =
            serde_json::from_str(pods_json).map_err(decode_err)?;
        self.state.write().tickets.set_ticket_pods(slug, pods);
        Ok(())
    }

    pub fn board_columns_json(&self) -> String {
        serde_json::to_string(self.state.read().tickets.get_board_columns()).unwrap_or_default()
    }

    pub fn labels_json(&self) -> String {
        serde_json::to_string(self.state.read().tickets.get_labels()).unwrap_or_default()
    }

    pub fn current_ticket_json(&self) -> JsValue {
        match self.state.read().tickets.get_current_ticket() {
            Some(t) => JsValue::from_str(&serde_json::to_string(t).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    pub fn replace_cached_tickets(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.set_tickets(req.tickets);
        Ok(())
    }

    pub fn insert_created_ticket(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertCreatedTicketRequest::decode(req_bytes).map_err(decode_err)?;
        let ticket = req.ticket.ok_or_else(|| JsValue::from_str("missing ticket"))?;
        self.state.write().tickets.add_ticket(ticket);
        Ok(())
    }

    pub fn patch_cached_ticket(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchCachedTicketRequest::decode(req_bytes).map_err(decode_err)?;
        let ticket = req.ticket.ok_or_else(|| JsValue::from_str("missing ticket"))?;
        self.state.write().tickets.update_ticket(&req.slug, ticket);
        Ok(())
    }

    pub fn apply_ticket_status_event(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyTicketStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.update_ticket_status(&req.slug, &req.status);
        Ok(())
    }

    pub fn apply_ticket_deleted_event(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyTicketDeletedEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.remove_ticket(&req.slug);
        Ok(())
    }

    pub fn replace_board_columns(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceBoardColumnsRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.set_board_columns(req.columns);
        Ok(())
    }

    pub fn append_board_column_tickets(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = AppendBoardColumnTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.append_column_tickets(&req.status, req.tickets);
        Ok(())
    }

    pub fn set_current_ticket(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentTicketRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.set_current_ticket(req.ticket);
        Ok(())
    }

    pub fn replace_cached_labels(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedLabelsRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.set_labels(req.labels);
        Ok(())
    }

    pub fn insert_created_label(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertCreatedLabelRequest::decode(req_bytes).map_err(decode_err)?;
        let label = req.label.ok_or_else(|| JsValue::from_str("missing label"))?;
        self.state.write().tickets.add_label(label);
        Ok(())
    }

    pub fn remove_cached_label(&self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = RemoveCachedLabelRequest::decode(req_bytes).map_err(decode_err)?;
        self.state.write().tickets.remove_label(req.id);
        Ok(())
    }

    pub fn filter_tickets(&self, req_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        let req = FilterTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        let search = if req.search.is_empty() { None } else { Some(req.search.as_str()) };
        let guard = self.state.read();
        let tickets: Vec<_> = guard.tickets
            .filter_tickets(search, &req.statuses, &req.priorities, &req.repository_ids)
            .into_iter().cloned().collect();
        let resp = FilterTicketsResponse { tickets };
        Ok(resp.encode_to_vec())
    }
}
