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

    // -------- Connect-RPC (binary wire) --------
    //
    // TS encodes the request via @bufbuild/protobuf .toBinary(), passes the
    // Uint8Array in, receives a Uint8Array back, decodes via .fromBinary().
    // No JSON intermediate; conventions §2.5 forbids it on the client.
    //
    // js_name is camelCase; the `Connect` suffix marks the migration lane so
    // the legacy JSON methods can coexist until the UI fully cuts over.

    #[wasm_bindgen(js_name = listSupportTicketsConnect)]
    pub async fn list_support_tickets_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.list_support_tickets_connect(request).await
    }

    #[wasm_bindgen(js_name = getSupportTicketConnect)]
    pub async fn get_support_ticket_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_support_ticket_connect(request).await
    }

    #[wasm_bindgen(js_name = getAttachmentUrlConnect)]
    pub async fn get_attachment_url_connect(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        self.0.get_attachment_url_connect(request).await
    }
}
