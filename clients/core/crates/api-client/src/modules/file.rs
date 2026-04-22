use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn presign_file_upload(
        &self,
        data: &PresignRequest,
    ) -> Result<PresignResponse, ApiError> {
        self.post(&self.org_path("/files/presign"), data).await
    }
}
