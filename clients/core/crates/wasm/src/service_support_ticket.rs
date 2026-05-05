use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_services::SupportTicketService;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSupportTicketService(pub(crate) SupportTicketService);

#[wasm_bindgen]
impl WasmSupportTicketService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self(SupportTicketService::new(client))
    }

    pub async fn list(
        &self, status: Option<String>, page: Option<u32>, page_size: Option<u32>,
    ) -> Result<String, String> {
        self.0.list(status, page, page_size).await
    }

    pub async fn get_detail(&self, id: i64) -> Result<String, String> {
        self.0.get_detail(id).await
    }

    pub async fn get_attachment_url(&self, id: i64) -> Result<String, String> {
        self.0.get_attachment_url(id).await
    }

    pub async fn create_ticket(
        &self, title: &str, category: &str, content: &str,
        priority: Option<String>, file_data: Vec<js_sys::Uint8Array>, file_names: Vec<String>,
    ) -> Result<String, String> {
        let bytes: Vec<Vec<u8>> = file_data.iter().map(|d| d.to_vec()).collect();
        self.0.create_ticket(title, category, content, priority, bytes, file_names).await
    }

    pub async fn add_message(
        &self, ticket_id: i64, content: &str,
        file_data: Vec<js_sys::Uint8Array>, file_names: Vec<String>,
    ) -> Result<String, String> {
        let bytes: Vec<Vec<u8>> = file_data.iter().map(|d| d.to_vec()).collect();
        self.0.add_message(ticket_id, content, bytes, file_names).await
    }
}
