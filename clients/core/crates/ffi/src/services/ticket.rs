use crate::core::AgentsMeshCore;
use crate::dto::{
    add_assignee_req, add_ticket_label_req, update_ticket_status_req, BoardResponseDto,
    CreateLabelRequestDto, CreateTicketRequestDto, LabelDto, LabelListResponseDto,
    PodListResponseDto, TicketDto, TicketListResponseDto, TicketStatusDto, UpdateLabelRequestDto,
    UpdateTicketRequestDto,
};
use crate::error::CoreError;

#[uniffi::export]
impl AgentsMeshCore {
    pub async fn list_tickets(
        &self,
        status: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<TicketListResponseDto, CoreError> {
        let resp = self.api.list_tickets(status.as_deref(), limit, offset).await?;
        Ok(resp.into())
    }

    pub async fn get_ticket(&self, slug: String) -> Result<TicketDto, CoreError> {
        let t = self.api.get_ticket(&slug).await?;
        Ok(t.into())
    }

    pub async fn create_ticket(
        &self,
        req: CreateTicketRequestDto,
    ) -> Result<TicketDto, CoreError> {
        let t = self.api.create_ticket(&req.into()).await?;
        Ok(t.into())
    }

    pub async fn update_ticket(
        &self,
        slug: String,
        req: UpdateTicketRequestDto,
    ) -> Result<TicketDto, CoreError> {
        let t = self.api.update_ticket(&slug, &req.into()).await?;
        Ok(t.into())
    }

    pub async fn delete_ticket(&self, slug: String) -> Result<(), CoreError> {
        self.api.delete_ticket(&slug).await?;
        Ok(())
    }

    pub async fn update_ticket_status(
        &self,
        slug: String,
        status: TicketStatusDto,
    ) -> Result<TicketDto, CoreError> {
        let t = self
            .api
            .update_ticket_status(&slug, &update_ticket_status_req(status))
            .await?;
        Ok(t.into())
    }

    pub async fn get_active_tickets(
        &self,
        limit: Option<u32>,
    ) -> Result<TicketListResponseDto, CoreError> {
        let resp = self.api.get_active_tickets(limit).await?;
        Ok(resp.into())
    }

    pub async fn get_ticket_board(
        &self,
        repository_id: Option<i64>,
    ) -> Result<BoardResponseDto, CoreError> {
        let resp = self.api.get_ticket_board(repository_id).await?;
        Ok(resp.into())
    }

    pub async fn get_sub_tickets(&self, slug: String) -> Result<TicketListResponseDto, CoreError> {
        let resp = self.api.get_sub_tickets(&slug).await?;
        Ok(resp.into())
    }

    pub async fn list_labels(
        &self,
        repository_id: Option<i64>,
    ) -> Result<LabelListResponseDto, CoreError> {
        let resp = self.api.list_labels(repository_id).await?;
        Ok(resp.into())
    }

    pub async fn create_label(&self, req: CreateLabelRequestDto) -> Result<LabelDto, CoreError> {
        let l = self.api.create_label(&req.into()).await?;
        Ok(l.into())
    }

    pub async fn update_label(
        &self,
        id: i64,
        req: UpdateLabelRequestDto,
    ) -> Result<LabelDto, CoreError> {
        let l = self.api.update_label(id, &req.into()).await?;
        Ok(l.into())
    }

    pub async fn delete_label(&self, id: i64) -> Result<(), CoreError> {
        self.api.delete_label(id).await?;
        Ok(())
    }

    pub async fn add_ticket_assignee(&self, slug: String, user_id: i64) -> Result<(), CoreError> {
        self.api
            .add_ticket_assignee(&slug, &add_assignee_req(user_id))
            .await?;
        Ok(())
    }

    pub async fn remove_ticket_assignee(
        &self,
        slug: String,
        user_id: i64,
    ) -> Result<(), CoreError> {
        self.api.remove_ticket_assignee(&slug, user_id).await?;
        Ok(())
    }

    pub async fn get_ticket_pods(
        &self,
        slug: String,
        active_only: Option<bool>,
    ) -> Result<PodListResponseDto, CoreError> {
        let resp = self.api.get_ticket_pods(&slug, active_only).await?;
        Ok(resp.into())
    }

    pub async fn add_ticket_label(&self, slug: String, label_id: i64) -> Result<(), CoreError> {
        self.api
            .add_ticket_label(&slug, &add_ticket_label_req(label_id))
            .await?;
        Ok(())
    }

    pub async fn remove_ticket_label(
        &self,
        slug: String,
        label_id: i64,
    ) -> Result<(), CoreError> {
        self.api.remove_ticket_label(&slug, label_id).await?;
        Ok(())
    }
}
