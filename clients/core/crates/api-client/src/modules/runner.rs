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
// Legacy REST — carve-outs without proto coverage.
// =============================================================================
//
// Each method below has no Connect-RPC equivalent and is preserved as REST:
//   - list_runner_pods: proto.pod.v1.ListPods has no runner_id filter yet;
//     runner-detail view (web + desktop) still drives the per-runner list
//     via this REST path.
//   - get_runner_auth_status / authorize_runner: registration bootstrap
//     (Tailscale-style interactive auth) lives outside the org-scoped
//     RunnerService — REST stays until the runner-mgmt RPCs land.

impl ApiClient {
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
