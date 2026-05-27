use agentsmesh_state::ticket_state::TicketState;
use agentsmesh_types::proto_ticket_state_v1::{
    ApplyTicketDeletedEventRequest, ApplyTicketStatusEventRequest,
    AppendBoardColumnTicketsRequest, FilterTicketsRequest, FilterTicketsResponse,
    InsertCreatedLabelRequest, InsertCreatedTicketRequest,
    PatchCachedTicketRequest, RemoveCachedLabelRequest,
    ReplaceBoardColumnsRequest, ReplaceCachedLabelsRequest,
    ReplaceCachedTicketsRequest, SetCurrentTicketRequest,
};
use prost::Message;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTicketState {
    inner: TicketState,
}

fn decode_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("decode: {e}"))
}

#[wasm_bindgen]
impl WasmTicketState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: TicketState::with_storage(crate::new_memory_backend()) }
    }

    // -------- Read accessors (kept JSON for ergonomic React consumers).
    // The renderer materialises these into typed views via JSON.parse;
    // a proto round-trip here costs ~2x for no business benefit because
    // the values are never crossed back across the boundary.

    pub fn tickets_json(&self) -> String {
        serde_json::to_string(self.inner.get_tickets()).unwrap_or_default()
    }

    pub fn board_columns_json(&self) -> String {
        serde_json::to_string(self.inner.get_board_columns()).unwrap_or_default()
    }

    pub fn labels_json(&self) -> String {
        serde_json::to_string(self.inner.get_labels()).unwrap_or_default()
    }

    pub fn current_ticket_json(&self) -> JsValue {
        match self.inner.get_current_ticket() {
            Some(t) => JsValue::from_str(&serde_json::to_string(t).unwrap_or_default()),
            None => JsValue::NULL,
        }
    }

    // -------- Proto bytes mutators.

    pub fn replace_cached_tickets(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_tickets(req.tickets);
        Ok(())
    }

    pub fn insert_created_ticket(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertCreatedTicketRequest::decode(req_bytes).map_err(decode_err)?;
        let ticket = req.ticket.ok_or_else(|| JsValue::from_str("missing ticket"))?;
        self.inner.add_ticket(ticket);
        Ok(())
    }

    pub fn patch_cached_ticket(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = PatchCachedTicketRequest::decode(req_bytes).map_err(decode_err)?;
        let ticket = req.ticket.ok_or_else(|| JsValue::from_str("missing ticket"))?;
        self.inner.update_ticket(&req.slug, ticket);
        Ok(())
    }

    pub fn apply_ticket_status_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyTicketStatusEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.update_ticket_status(&req.slug, &req.status);
        Ok(())
    }

    pub fn apply_ticket_deleted_event(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ApplyTicketDeletedEventRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.remove_ticket(&req.slug);
        Ok(())
    }

    pub fn replace_board_columns(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceBoardColumnsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_board_columns(req.columns);
        Ok(())
    }

    pub fn append_board_column_tickets(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = AppendBoardColumnTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.append_column_tickets(&req.status, req.tickets);
        Ok(())
    }

    pub fn set_current_ticket(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = SetCurrentTicketRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_current_ticket(req.ticket);
        Ok(())
    }

    pub fn replace_cached_labels(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = ReplaceCachedLabelsRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.set_labels(req.labels);
        Ok(())
    }

    pub fn insert_created_label(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = InsertCreatedLabelRequest::decode(req_bytes).map_err(decode_err)?;
        let label = req.label.ok_or_else(|| JsValue::from_str("missing label"))?;
        self.inner.add_label(label);
        Ok(())
    }

    pub fn remove_cached_label(&mut self, req_bytes: &[u8]) -> Result<(), JsValue> {
        let req = RemoveCachedLabelRequest::decode(req_bytes).map_err(decode_err)?;
        self.inner.remove_label(req.id);
        Ok(())
    }

    pub fn filter_tickets(&self, req_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        let req = FilterTicketsRequest::decode(req_bytes).map_err(decode_err)?;
        let search = if req.search.is_empty() { None } else { Some(req.search.as_str()) };
        let tickets: Vec<_> = self.inner
            .filter_tickets(search, &req.statuses, &req.priorities, &req.repository_ids)
            .into_iter().cloned().collect();
        let resp = FilterTicketsResponse { tickets };
        Ok(resp.encode_to_vec())
    }
}
