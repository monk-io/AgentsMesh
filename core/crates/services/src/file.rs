use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;

pub struct FileService {
    client: Arc<ApiClient>,
}

impl FileService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn presign_upload(&self, json: &str) -> Result<String, String> {
        let req: PresignRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.presign_file_upload(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn upload_file(
        &self, file_data: Vec<u8>, filename: &str, content_type: &str,
    ) -> Result<String, String> {
        let size = file_data.len();
        let req = PresignRequest {
            filename: filename.to_string(),
            content_type: content_type.to_string(),
            size: size as i64,
        };
        let presign = self.client.presign_file_upload(&req).await.map_err(crate::wire)?;

        self.client.put_raw_bytes(&presign.put_url, content_type, file_data)
            .await.map_err(crate::wire)?;
        Ok(presign.get_url)
    }
}
