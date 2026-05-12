use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_runner_api_v1 as runner_proto;

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in
// backend/internal/api/connect/runner/. Procedure paths derive from
// `proto.runner_api.v1.RunnerService.<Method>` (conventions §12).

impl ApiClient {
    pub async fn list_runners_connect(
        &self,
        req: &runner_proto::ListRunnersRequest,
    ) -> Result<runner_proto::ListRunnersResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/ListRunners",
            req,
        )
        .await
    }

    pub async fn list_available_runners_connect(
        &self,
        req: &runner_proto::ListAvailableRunnersRequest,
    ) -> Result<runner_proto::ListAvailableRunnersResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/ListAvailableRunners",
            req,
        )
        .await
    }

    pub async fn get_runner_connect(
        &self,
        req: &runner_proto::GetRunnerRequest,
    ) -> Result<runner_proto::GetRunnerResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/GetRunner",
            req,
        )
        .await
    }

    pub async fn update_runner_connect(
        &self,
        req: &runner_proto::UpdateRunnerRequest,
    ) -> Result<runner_proto::Runner, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/UpdateRunner",
            req,
        )
        .await
    }

    pub async fn delete_runner_connect(
        &self,
        req: &runner_proto::DeleteRunnerRequest,
    ) -> Result<runner_proto::DeleteRunnerResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/DeleteRunner",
            req,
        )
        .await
    }

    pub async fn upgrade_runner_connect(
        &self,
        req: &runner_proto::UpgradeRunnerRequest,
    ) -> Result<runner_proto::UpgradeRunnerResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/UpgradeRunner",
            req,
        )
        .await
    }

    pub async fn request_log_upload_connect(
        &self,
        req: &runner_proto::RequestLogUploadRequest,
    ) -> Result<runner_proto::RequestLogUploadResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/RequestLogUpload",
            req,
        )
        .await
    }

    pub async fn list_runner_logs_connect(
        &self,
        req: &runner_proto::ListRunnerLogsRequest,
    ) -> Result<runner_proto::ListRunnerLogsResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/ListRunnerLogs",
            req,
        )
        .await
    }

    pub async fn query_sandboxes_connect(
        &self,
        req: &runner_proto::QuerySandboxesRequest,
    ) -> Result<runner_proto::QuerySandboxesResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/QuerySandboxes",
            req,
        )
        .await
    }

    pub async fn create_runner_token_connect(
        &self,
        req: &runner_proto::CreateRunnerTokenRequest,
    ) -> Result<runner_proto::RunnerToken, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/CreateRunnerToken",
            req,
        )
        .await
    }

    pub async fn list_runner_tokens_connect(
        &self,
        req: &runner_proto::ListRunnerTokensRequest,
    ) -> Result<runner_proto::ListRunnerTokensResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/ListRunnerTokens",
            req,
        )
        .await
    }

    pub async fn delete_runner_token_connect(
        &self,
        req: &runner_proto::DeleteRunnerTokenRequest,
    ) -> Result<runner_proto::DeleteRunnerTokenResponse, ApiError> {
        connect_call(
            self,
            "/proto.runner_api.v1.RunnerService/DeleteRunnerToken",
            req,
        )
        .await
    }
}

// =============================================================================
// Legacy REST methods — preserved for dual-track migration.
// =============================================================================

impl ApiClient {
    pub async fn list_runners(
        &self,
        status: Option<&str>,
    ) -> Result<RunnerListResponse, ApiError> {
        let mut path = self.org_path("/runners");
        if let Some(s) = status {
            path = format!("{path}?status={s}");
        }
        self.get(&path).await
    }

    pub async fn list_available_runners(&self) -> Result<RunnerListResponse, ApiError> {
        self.get(&self.org_path("/runners/available")).await
    }

    pub async fn get_runner(&self, id: i64) -> Result<RunnerDetailResponse, ApiError> {
        self.get(&self.org_path(&format!("/runners/{id}"))).await
    }

    pub async fn update_runner(
        &self,
        id: i64,
        data: &UpdateRunnerRequest,
    ) -> Result<Runner, ApiError> {
        self.put_resource(&self.org_path(&format!("/runners/{id}")), data, "runner").await
    }

    pub async fn delete_runner(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/runners/{id}"))).await
    }

    pub async fn create_runner_token(
        &self,
        data: &CreateRunnerTokenRequest,
    ) -> Result<GRPCRegistrationToken, ApiError> {
        self.post(&self.org_path("/runners/grpc/tokens"), data)
            .await
    }

    pub async fn list_runner_tokens(&self) -> Result<RunnerTokenListResponse, ApiError> {
        self.get(&self.org_path("/runners/grpc/tokens")).await
    }

    pub async fn delete_runner_token(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path(&format!("/runners/grpc/tokens/{id}")))
            .await
    }

    pub async fn list_runner_pods(
        &self,
        id: i64,
        status: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<PodListResponse, ApiError> {
        let mut path = self.org_path(&format!("/runners/{id}/pods"));
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

    pub async fn query_runner_sandboxes(
        &self,
        id: i64,
        data: &SandboxQueryRequest,
    ) -> Result<SandboxQueryResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/runners/{id}/sandboxes/query")),
            data,
        )
        .await
    }

    pub async fn upgrade_runner(
        &self,
        id: i64,
        data: &UpgradeRunnerRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/runners/{id}/upgrade")),
            data,
        )
        .await
    }

    pub async fn request_runner_log_upload(
        &self,
        id: i64,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path(&format!("/runners/{id}/logs/upload")),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn list_runner_logs(&self, id: i64) -> Result<RunnerLogListResponse, ApiError> {
        self.get(&self.org_path(&format!("/runners/{id}/logs")))
            .await
    }

    pub async fn get_runner_auth_status(
        &self,
        auth_key: &str,
    ) -> Result<RunnerAuthStatus, ApiError> {
        self.get(&self.org_path(&format!("/runners/auth/{auth_key}")))
            .await
    }

    pub async fn authorize_runner(
        &self,
        data: &AuthorizeRunnerRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path("/runners/grpc/authorize"), data).await
    }
}
