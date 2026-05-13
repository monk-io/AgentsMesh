use std::sync::Arc;
use std::sync::RwLock;

use agentsmesh_api_client::ApiClient;
use agentsmesh_state::ticket_state::TicketState;
use agentsmesh_types::proto_ticket_v1 as ticket_proto;
use agentsmesh_types::{
    Ticket, TicketStatus, TicketPriority, Label, BoardColumn,
    CreateTicketRequest, UpdateTicketRequest,
};
use prost::Message;

use crate::parse_status;

pub struct TicketService {
    client: Arc<ApiClient>,
    state: RwLock<TicketState>,
}

impl TicketService {
    pub fn new(client: Arc<ApiClient>, state: TicketState) -> Self {
        Self { client, state: RwLock::new(state) }
    }

    pub fn tickets_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_tickets()).unwrap_or_default()
    }

    pub fn get_ticket_by_slug_json(&self, slug: &str) -> Option<String> {
        self.state.read().unwrap().get_ticket_by_slug(slug)
            .map(|t| serde_json::to_string(t).unwrap_or_default())
    }

    pub fn current_ticket_json(&self) -> Option<String> {
        self.state.read().unwrap().get_current_ticket()
            .map(|t| serde_json::to_string(t).unwrap_or_default())
    }

    pub fn board_columns_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_board_columns()).unwrap_or_default()
    }

    pub fn labels_json(&self) -> String {
        serde_json::to_string(self.state.read().unwrap().get_labels()).unwrap_or_default()
    }

    pub fn filter_tickets_json(
        &self, search: &str, statuses_json: &str,
        priorities_json: &str, repository_ids_json: &str,
    ) -> String {
        let statuses: Vec<TicketStatus> = serde_json::from_str(statuses_json).unwrap_or_default();
        let priorities: Vec<TicketPriority> = serde_json::from_str(priorities_json).unwrap_or_default();
        let repo_ids: Vec<i64> = serde_json::from_str(repository_ids_json).unwrap_or_default();
        let s = if search.is_empty() { None } else { Some(search) };
        let binding = self.state.read().unwrap();
        let filtered = binding.filter_tickets(s, &statuses, &priorities, &repo_ids);
        serde_json::to_string(&filtered).unwrap_or_default()
    }

    pub fn set_tickets(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<Ticket>>(json) {
            self.state.write().unwrap().set_tickets(v);
        }
    }

    pub fn add_ticket(&self, json: &str) {
        if let Ok(t) = serde_json::from_str::<Ticket>(json) {
            self.state.write().unwrap().add_ticket(t);
        }
    }

    pub fn update_ticket_local(&self, slug: &str, json: &str) {
        if let Ok(t) = serde_json::from_str::<Ticket>(json) {
            self.state.write().unwrap().update_ticket(slug, t);
        }
    }

    pub fn update_ticket_status_local(&self, slug: &str, status: &str) {
        let parsed = parse_status::<TicketStatus>(status);
        self.state.write().unwrap().update_ticket_status(slug, parsed);
    }

    pub fn remove_ticket(&self, slug: &str) {
        self.state.write().unwrap().remove_ticket(slug);
    }

    pub fn set_current_ticket(&self, json: &str) {
        let t = if json.is_empty() { None } else { serde_json::from_str::<Ticket>(json).ok() };
        self.state.write().unwrap().set_current_ticket(t);
    }

    pub fn set_board_columns(&self, json: &str) {
        if let Ok(cols) = serde_json::from_str::<Vec<BoardColumn>>(json) {
            self.state.write().unwrap().set_board_columns(cols);
        }
    }

    pub fn append_column_tickets(&self, status: &str, json: &str) {
        let parsed = parse_status::<TicketStatus>(status);
        if let Ok(tickets) = serde_json::from_str::<Vec<Ticket>>(json) {
            self.state.write().unwrap().append_column_tickets(parsed, tickets);
        }
    }

    pub fn set_labels(&self, json: &str) {
        if let Ok(v) = serde_json::from_str::<Vec<Label>>(json) {
            self.state.write().unwrap().set_labels(v);
        }
    }

    pub fn add_label(&self, json: &str) {
        if let Ok(l) = serde_json::from_str::<Label>(json) {
            self.state.write().unwrap().add_label(l);
        }
    }

    pub fn remove_label(&self, id: f64) {
        self.state.write().unwrap().remove_label(id as i64);
    }

    pub async fn fetch_tickets(
        &self, status: Option<String>, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let req = ticket_proto::ListTicketsRequest {
            org_slug: self.client.current_org_slug(),
            repository_id: None,
            status,
            priority: None,
            assignee_id: None,
            labels: vec![],
            query: None,
            offset: offset.map(|v| v as i32),
            limit: limit.map(|v| v as i32),
        };
        let resp = self.client.list_tickets_connect(&req).await.map_err(crate::wire)?;
        let total = resp.total;
        let resp_limit = resp.limit;
        let resp_offset = resp.offset;
        let tickets: Vec<Ticket> = resp.items.into_iter().map(crate::proto_convert::ticket::from_proto).collect();
        self.state.write().unwrap().set_tickets(tickets.clone());
        let envelope = serde_json::json!({
            "tickets": tickets,
            "total": total,
            "limit": resp_limit,
            "offset": resp_offset,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn fetch_board(&self, repository_id: Option<i64>) -> Result<String, String> {
        let req = ticket_proto::GetBoardRequest {
            org_slug: self.client.current_org_slug(),
            repository_id,
            limit: None,
            priority: None,
            assignee_id: None,
            query: None,
        };
        let resp = self.client.get_board_connect(&req).await.map_err(crate::wire)?;
        let columns: Vec<BoardColumn> = resp.columns.into_iter().map(crate::proto_convert::ticket::board_column_from_proto).collect();
        self.state.write().unwrap().set_board_columns(columns.clone());
        let envelope = serde_json::json!({ "columns": columns });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn load_more_column(
        &self, status: &str, offset: u32, limit: u32,
    ) -> Result<String, String> {
        let req = ticket_proto::ListTicketsRequest {
            org_slug: self.client.current_org_slug(),
            repository_id: None,
            status: Some(status.to_string()),
            priority: None,
            assignee_id: None,
            labels: vec![],
            query: None,
            offset: Some(offset as i32),
            limit: Some(limit as i32),
        };
        let resp = self.client.list_tickets_connect(&req).await.map_err(crate::wire)?;
        let total = resp.total;
        let tickets: Vec<Ticket> = resp.items.into_iter().map(crate::proto_convert::ticket::from_proto).collect();
        let parsed = parse_status::<TicketStatus>(status);
        self.state.write().unwrap().append_column_tickets(parsed, tickets.clone());
        let envelope = serde_json::json!({
            "tickets": tickets,
            "total": total,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn fetch_ticket(&self, slug: &str) -> Result<String, String> {
        let req = ticket_proto::GetTicketRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
        };
        let resp = self.client.get_ticket_connect(&req).await.map_err(crate::wire)?;
        let ticket = crate::proto_convert::ticket::from_proto(resp);
        self.state.write().unwrap().set_current_ticket(Some(ticket.clone()));
        serde_json::to_string(&ticket).map_err(crate::wire)
    }

    pub async fn create_ticket(&self, request_json: &str) -> Result<String, String> {
        let req_legacy: CreateTicketRequest = serde_json::from_str(request_json)
            .map_err(crate::wire)?;
        let req = ticket_proto::CreateTicketRequest {
            org_slug: self.client.current_org_slug(),
            title: req_legacy.title,
            content: req_legacy.content,
            status: None,
            priority: req_legacy.priority.and_then(|p| serde_json::to_value(p).ok().and_then(|v| v.as_str().map(String::from))),
            repository_id: req_legacy.repository_id,
            assignee_ids: req_legacy.assignee_ids.unwrap_or_default(),
            labels: req_legacy.labels.unwrap_or_default().into_iter().map(|id| id.to_string()).collect(),
            parent_ticket_slug: req_legacy.parent_slug,
            due_date: None,
        };
        let resp = self.client.create_ticket_connect(&req).await.map_err(crate::wire)?;
        let ticket = crate::proto_convert::ticket::from_proto(resp);
        self.state.write().unwrap().add_ticket(ticket.clone());
        serde_json::to_string(&ticket).map_err(crate::wire)
    }

    pub async fn update_ticket(&self, slug: &str, request_json: &str) -> Result<String, String> {
        let req_legacy: UpdateTicketRequest = serde_json::from_str(request_json)
            .map_err(crate::wire)?;
        let req = ticket_proto::UpdateTicketRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
            title: req_legacy.title,
            content: req_legacy.content,
            status: None,
            priority: req_legacy.priority.and_then(|p| serde_json::to_value(p).ok().and_then(|v| v.as_str().map(String::from))),
            repository_id: req_legacy.repository_id,
            assignee_ids: vec![],
            labels: vec![],
            due_date: None,
        };
        let resp = self.client.update_ticket_connect(&req).await.map_err(crate::wire)?;
        let ticket = crate::proto_convert::ticket::from_proto(resp);
        self.state.write().unwrap().update_ticket(slug, ticket.clone());
        serde_json::to_string(&ticket).map_err(crate::wire)
    }

    pub async fn delete_ticket(&self, slug: &str) -> Result<(), String> {
        let req = ticket_proto::DeleteTicketRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
        };
        self.client.delete_ticket_connect(&req).await.map_err(crate::wire)?;
        self.state.write().unwrap().remove_ticket(slug);
        Ok(())
    }

    pub async fn update_ticket_status(&self, slug: &str, status: &str) -> Result<String, String> {
        let req = ticket_proto::UpdateTicketStatusRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
            status: status.to_string(),
        };
        self.client.update_ticket_status_connect(&req).await.map_err(crate::wire)?;
        // proto.ticket.v1 UpdateTicketStatus returns an empty response; re-fetch
        // the ticket so the legacy method contract (returns Ticket JSON) holds.
        let get_req = ticket_proto::GetTicketRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
        };
        let ticket_proto_msg = self.client.get_ticket_connect(&get_req).await.map_err(crate::wire)?;
        let ticket = crate::proto_convert::ticket::from_proto(ticket_proto_msg);
        self.state.write().unwrap().update_ticket(slug, ticket.clone());
        serde_json::to_string(&ticket).map_err(crate::wire)
    }

    pub async fn fetch_labels(&self, repository_id: Option<i64>) -> Result<String, String> {
        let req = ticket_proto::ListLabelsRequest {
            org_slug: self.client.current_org_slug(),
            repository_id,
        };
        let resp = self.client.list_labels_connect(&req).await.map_err(crate::wire)?;
        let labels: Vec<Label> = resp.items.into_iter().map(crate::proto_convert::ticket::label_from_proto).collect();
        self.state.write().unwrap().set_labels(labels.clone());
        serde_json::to_string(&labels).map_err(crate::wire)
    }

    pub async fn create_label(&self, name: &str, color: &str, repository_id: Option<i64>) -> Result<String, String> {
        let req = ticket_proto::CreateLabelRequest {
            org_slug: self.client.current_org_slug(),
            name: name.to_string(),
            color: color.to_string(),
            repository_id,
        };
        let resp = self.client.create_label_connect(&req).await.map_err(crate::wire)?;
        let label = crate::proto_convert::ticket::label_from_proto(resp);
        self.state.write().unwrap().add_label(label.clone());
        serde_json::to_string(&label).map_err(crate::wire)
    }

    pub async fn delete_label(&self, id: f64) -> Result<(), String> {
        let req = ticket_proto::DeleteLabelRequest {
            org_slug: self.client.current_org_slug(),
            id: id as i64,
        };
        self.client.delete_label_connect(&req).await.map_err(crate::wire)?;
        self.state.write().unwrap().remove_label(id as i64);
        Ok(())
    }

    pub async fn get_ticket_pods(
        &self, slug: &str, active_only: Option<bool>,
    ) -> Result<String, String> {
        // proto.ticket.v1 does not own ticket→pod lookup — that's MeshService
        // (see mesh.go). Stay on REST until MeshService migrates.
        let resp = self.client
            .get_ticket_pods(slug, active_only)
            .await.map_err(crate::wire)?;
        self.state.write().unwrap().set_ticket_pods(slug, resp.pods.clone());
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub fn ticket_pods_json(&self, slug: &str) -> String {
        let pods = self.state.read().unwrap().get_ticket_pods(slug);
        serde_json::to_string(&pods).unwrap_or_else(|_| "[]".into())
    }

    pub async fn get_sub_tickets(&self, slug: &str) -> Result<String, String> {
        let req = ticket_proto::GetSubTicketsRequest {
            org_slug: self.client.current_org_slug(),
            ticket_slug: slug.to_string(),
        };
        let resp = self.client.get_sub_tickets_connect(&req).await.map_err(crate::wire)?;
        let tickets: Vec<Ticket> = resp.items.into_iter().map(crate::proto_convert::ticket::from_proto).collect();
        let envelope = serde_json::json!({ "tickets": tickets });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }
}

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// Each `*_connect` method takes prost-encoded bytes and returns
// prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
// surface. Caller (TS) encodes via @bufbuild/protobuf .toBinary() and
// decodes via .fromBinary().

impl TicketService {
    pub async fn list_tickets_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::ListTicketsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_tickets request: {e}"))?;
        let resp = self.client.list_tickets_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_ticket_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::GetTicketRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_ticket request: {e}"))?;
        let resp = self.client.get_ticket_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_ticket_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::CreateTicketRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_ticket request: {e}"))?;
        let resp = self.client.create_ticket_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_ticket_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::UpdateTicketRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_ticket request: {e}"))?;
        let resp = self.client.update_ticket_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_ticket_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::DeleteTicketRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_ticket request: {e}"))?;
        let resp = self.client.delete_ticket_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_ticket_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::UpdateTicketStatusRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_ticket_status request: {e}"))?;
        let resp = self.client.update_ticket_status_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_active_tickets_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::GetActiveTicketsRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_active_tickets request: {e}"))?;
        let resp = self.client.get_active_tickets_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_board_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::GetBoardRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_board request: {e}"))?;
        let resp = self.client.get_board_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_sub_tickets_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::GetSubTicketsRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_sub_tickets request: {e}"))?;
        let resp = self.client.get_sub_tickets_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn add_assignee_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::AddAssigneeRequest::decode(request_bytes)
            .map_err(|e| format!("decode add_assignee request: {e}"))?;
        let resp = self.client.add_assignee_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn remove_assignee_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::RemoveAssigneeRequest::decode(request_bytes)
            .map_err(|e| format!("decode remove_assignee request: {e}"))?;
        let resp = self.client.remove_assignee_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_labels_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::ListLabelsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_labels request: {e}"))?;
        let resp = self.client.list_labels_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_label_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::CreateLabelRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_label request: {e}"))?;
        let resp = self.client.create_label_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_label_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::UpdateLabelRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_label request: {e}"))?;
        let resp = self.client.update_label_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_label_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::DeleteLabelRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_label request: {e}"))?;
        let resp = self.client.delete_label_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn add_label_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::AddLabelRequest::decode(request_bytes)
            .map_err(|e| format!("decode add_label request: {e}"))?;
        let resp = self.client.add_label_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn remove_label_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ticket_proto::RemoveLabelRequest::decode(request_bytes)
            .map_err(|e| format!("decode remove_label request: {e}"))?;
        let resp = self.client.remove_label_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
