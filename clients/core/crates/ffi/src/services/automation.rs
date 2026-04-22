use crate::core::AgentsMeshCore;
use crate::dto::{
    approve_autopilot_req, AutopilotControllerDto, AutopilotIterationListResponseDto,
    AutopilotListResponseDto, CreateAutopilotRequestDto, CreateLoopRequestDto, LoopDataDto,
    LoopListResponseDto, LoopRunDataDto, LoopRunListResponseDto, UpdateLoopRequestDto,
};
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    // ── Autopilot ─────────────────────────────────────────

    pub async fn list_autopilots(&self) -> Result<AutopilotListResponseDto, CoreError> {
        let resp = self.api.list_autopilots().await?;
        Ok(resp.into())
    }

    pub async fn get_autopilot(&self, key: String) -> Result<AutopilotControllerDto, CoreError> {
        let c = self.api.get_autopilot(&key).await?;
        Ok(c.into())
    }

    pub async fn create_autopilot(
        &self,
        req: CreateAutopilotRequestDto,
    ) -> Result<AutopilotControllerDto, CoreError> {
        let c = self.api.create_autopilot(&req.into()).await?;
        Ok(c.into())
    }

    pub async fn pause_autopilot(&self, key: String) -> Result<(), CoreError> {
        self.api.pause_autopilot(&key).await?;
        Ok(())
    }

    pub async fn resume_autopilot(&self, key: String) -> Result<(), CoreError> {
        self.api.resume_autopilot(&key).await?;
        Ok(())
    }

    pub async fn stop_autopilot(&self, key: String) -> Result<(), CoreError> {
        self.api.stop_autopilot(&key).await?;
        Ok(())
    }

    pub async fn approve_autopilot(
        &self,
        key: String,
        continue_execution: Option<bool>,
        additional_iterations: Option<i64>,
    ) -> Result<(), CoreError> {
        self.api
            .approve_autopilot(
                &key,
                &approve_autopilot_req(continue_execution, additional_iterations),
            )
            .await?;
        Ok(())
    }

    pub async fn takeover_autopilot(&self, key: String) -> Result<(), CoreError> {
        self.api.takeover_autopilot(&key).await?;
        Ok(())
    }

    pub async fn handback_autopilot(&self, key: String) -> Result<(), CoreError> {
        self.api.handback_autopilot(&key).await?;
        Ok(())
    }

    pub async fn get_autopilot_iterations(
        &self,
        key: String,
    ) -> Result<AutopilotIterationListResponseDto, CoreError> {
        let resp = self.api.get_autopilot_iterations(&key).await?;
        Ok(resp.into())
    }

    // ── Loop ──────────────────────────────────────────────

    pub async fn list_loops(
        &self,
        status: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LoopListResponseDto, CoreError> {
        let resp = self.api.list_loops(status.as_deref(), limit, offset).await?;
        Ok(resp.into())
    }

    pub async fn get_loop(&self, slug: String) -> Result<LoopDataDto, CoreError> {
        let l = self.api.get_loop(&slug).await?;
        Ok(l.into())
    }

    pub async fn create_loop(&self, req: CreateLoopRequestDto) -> Result<LoopDataDto, CoreError> {
        let l = self.api.create_loop(&req.into()).await?;
        Ok(l.into())
    }

    pub async fn update_loop(
        &self,
        slug: String,
        req: UpdateLoopRequestDto,
    ) -> Result<LoopDataDto, CoreError> {
        let l = self.api.update_loop(&slug, &req.into()).await?;
        Ok(l.into())
    }

    pub async fn delete_loop(&self, slug: String) -> Result<(), CoreError> {
        self.api.delete_loop(&slug).await?;
        Ok(())
    }

    pub async fn enable_loop(&self, slug: String) -> Result<(), CoreError> {
        self.api.enable_loop(&slug).await?;
        Ok(())
    }

    pub async fn disable_loop(&self, slug: String) -> Result<(), CoreError> {
        self.api.disable_loop(&slug).await?;
        Ok(())
    }

    pub async fn trigger_loop(&self, slug: String) -> Result<LoopRunDataDto, CoreError> {
        let run = self.api.trigger_loop(&slug).await?;
        Ok(run.into())
    }

    pub async fn list_loop_runs(
        &self,
        slug: String,
        status: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LoopRunListResponseDto, CoreError> {
        let resp = self
            .api
            .list_loop_runs(&slug, status.as_deref(), limit, offset)
            .await?;
        Ok(resp.into())
    }

    pub async fn get_loop_run(
        &self,
        slug: String,
        run_id: i64,
    ) -> Result<LoopRunDataDto, CoreError> {
        let run = self.api.get_loop_run(&slug, run_id).await?;
        Ok(run.into())
    }

    pub async fn cancel_loop_run(&self, slug: String, run_id: i64) -> Result<(), CoreError> {
        self.api.cancel_loop_run(&slug, run_id).await?;
        Ok(())
    }
}
