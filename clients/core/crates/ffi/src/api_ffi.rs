use crate::core::AgentsMeshCore;
use crate::error::CoreError;

#[uniffi::export(async_runtime = "tokio")]
impl AgentsMeshCore {
    pub async fn api_get(&self, endpoint: String) -> Result<String, CoreError> {
        let value: serde_json::Value = self.api.get(&endpoint).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_post(
        &self,
        endpoint: String,
        body: String,
    ) -> Result<String, CoreError> {
        let body: serde_json::Value = serde_json::from_str(&body)?;
        let value: serde_json::Value = self.api.post(&endpoint, &body).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_put(
        &self,
        endpoint: String,
        body: String,
    ) -> Result<String, CoreError> {
        let body: serde_json::Value = serde_json::from_str(&body)?;
        let value: serde_json::Value = self.api.put(&endpoint, &body).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_patch(
        &self,
        endpoint: String,
        body: String,
    ) -> Result<String, CoreError> {
        let body: serde_json::Value = serde_json::from_str(&body)?;
        let value: serde_json::Value = self.api.patch(&endpoint, &body).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_delete(&self, endpoint: String) -> Result<String, CoreError> {
        let value: serde_json::Value = self.api.delete(&endpoint).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_public_get(&self, endpoint: String) -> Result<String, CoreError> {
        let value: serde_json::Value = self.api.public_get(&endpoint).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub async fn api_public_post(
        &self,
        endpoint: String,
        body: String,
    ) -> Result<String, CoreError> {
        let body: serde_json::Value = serde_json::from_str(&body)?;
        let value: serde_json::Value = self.api.public_post(&endpoint, &body).await?;
        Ok(serde_json::to_string(&value)?)
    }

    pub fn api_org_path(&self, path: String) -> String {
        self.api.org_path(&path)
    }
}
