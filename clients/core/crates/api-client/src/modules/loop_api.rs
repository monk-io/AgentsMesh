use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn list_loops(
        &self,
        status: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LoopListResponse, ApiError> {
        let mut path = self.org_path("/loops");
        let mut params = Vec::new();
        if let Some(s) = status {
            params.push(format!("status={s}"));
        }
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_loop(&self, slug: &str) -> Result<LoopData, ApiError> {
        self.get_resource(&self.org_path(&format!("/loops/{slug}")), "loop").await
    }

    pub async fn create_loop(&self, data: &CreateLoopRequest) -> Result<LoopData, ApiError> {
        self.post_resource(&self.org_path("/loops"), data, "loop").await
    }

    pub async fn update_loop(
        &self,
        slug: &str,
        data: &UpdateLoopRequest,
    ) -> Result<LoopData, ApiError> {
        self.put_resource(&self.org_path(&format!("/loops/{slug}")), data, "loop").await
    }

    pub async fn delete_loop(&self, slug: &str) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/loops/{slug}")))
            .await
    }

    pub async fn enable_loop(&self, slug: &str) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/loops/{slug}/enable")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn disable_loop(&self, slug: &str) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/loops/{slug}/disable")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn trigger_loop(&self, slug: &str) -> Result<LoopRunData, ApiError> {
        self.post(
            &self.org_path(&format!("/loops/{slug}/trigger")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn list_loop_runs(
        &self,
        slug: &str,
        status: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LoopRunListResponse, ApiError> {
        let mut path = self.org_path(&format!("/loops/{slug}/runs"));
        let mut params = Vec::new();
        if let Some(s) = status {
            params.push(format!("status={s}"));
        }
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_loop_run(
        &self,
        slug: &str,
        run_id: i64,
    ) -> Result<LoopRunData, ApiError> {
        self.get_resource(&self.org_path(&format!("/loops/{slug}/runs/{run_id}")), "run").await
    }

    pub async fn cancel_loop_run(
        &self,
        slug: &str,
        run_id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/loops/{slug}/runs/{run_id}/cancel")),
            &serde_json::json!({}),
        )
        .await
    }
}
