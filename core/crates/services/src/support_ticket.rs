use std::sync::Arc;

use agentsmesh_api_client::ApiClient;

pub struct SupportTicketService {
    client: Arc<ApiClient>,
}

impl SupportTicketService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn list(
        &self, status: Option<String>, page: Option<u32>, page_size: Option<u32>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_support_tickets(status.as_deref(), page, page_size)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_detail(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_support_ticket_detail(id)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn get_attachment_url(&self, id: i64) -> Result<String, String> {
        let resp = self.client
            .get_support_ticket_attachment_url(id)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn create_ticket(
        &self, title: &str, category: &str, content: &str,
        priority: Option<String>, file_data: Vec<Vec<u8>>, file_names: Vec<String>,
    ) -> Result<String, String> {
        let mut form = reqwest::multipart::Form::new()
            .text("title", title.to_string())
            .text("category", category.to_string())
            .text("content", content.to_string());
        if let Some(p) = priority { form = form.text("priority", p); }
        for (i, (data, name)) in file_data.into_iter().zip(file_names.iter()).enumerate() {
            let part = reqwest::multipart::Part::bytes(data).file_name(name.clone());
            form = form.part(format!("files[{i}]"), part);
        }
        let resp = self.client
            .post_multipart::<serde_json::Value>("/api/v1/support-tickets", form)
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }

    pub async fn add_message(
        &self, ticket_id: i64, content: &str,
        file_data: Vec<Vec<u8>>, file_names: Vec<String>,
    ) -> Result<String, String> {
        let mut form = reqwest::multipart::Form::new()
            .text("content", content.to_string());
        for (i, (data, name)) in file_data.into_iter().zip(file_names.iter()).enumerate() {
            let part = reqwest::multipart::Part::bytes(data).file_name(name.clone());
            form = form.part(format!("files[{i}]"), part);
        }
        let resp = self.client
            .post_multipart::<serde_json::Value>(
                &format!("/api/v1/support-tickets/{ticket_id}/messages"), form,
            )
            .await.map_err(|e| e.to_string())?;
        serde_json::to_string(&resp).map_err(|e| e.to_string())
    }
}
