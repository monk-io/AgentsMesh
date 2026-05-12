use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmBillingService {
    client: Arc<ApiClient>,
}

#[wasm_bindgen]
impl WasmBillingService {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_overview(&self) -> Result<String, String> {
        let resp = self.client.get_billing_overview().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_subscription(&self) -> Result<String, String> {
        let resp = self.client.get_billing_subscription().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn create_subscription(&self, json: &str) -> Result<String, String> {
        let req: CreateSubscriptionRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.create_billing_subscription(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn cancel_subscription(&self) -> Result<String, String> {
        let resp = self.client.cancel_billing_subscription().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn update_subscription(&self, json: &str) -> Result<String, String> {
        let req: UpdateSubscriptionRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.update_billing_subscription(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn list_plans(&self) -> Result<String, String> {
        let resp = self.client.list_billing_plans().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_usage(&self, usage_type: Option<String>) -> Result<String, String> {
        let resp = self.client
            .get_billing_usage(usage_type.as_deref())
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn check_quota(&self, resource: &str, amount: Option<u32>) -> Result<String, String> {
        let resp = self.client
            .check_billing_quota(resource, amount)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn create_checkout(&self, json: &str) -> Result<String, String> {
        let req: CheckoutRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.create_billing_checkout(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_checkout_status(&self, order_no: &str) -> Result<String, String> {
        let resp = self.client
            .get_billing_checkout_status(order_no)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn request_cancel(&self, json: &str) -> Result<String, String> {
        let req: CancelSubscriptionRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.request_cancel_subscription(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn reactivate(&self) -> Result<String, String> {
        let resp = self.client.reactivate_subscription().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn upgrade(&self, json: &str) -> Result<String, String> {
        let req: UpgradeSubscriptionRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.upgrade_subscription(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn change_cycle(&self, json: &str) -> Result<String, String> {
        let req: ChangeBillingCycleRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.change_billing_cycle(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn update_auto_renew(&self, json: &str) -> Result<String, String> {
        let req: UpdateAutoRenewRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.update_auto_renew(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_seat_usage(&self) -> Result<String, String> {
        let resp = self.client.get_seat_usage().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn purchase_seats(&self, json: &str) -> Result<String, String> {
        let req: PurchaseSeatsRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.purchase_seats(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn list_invoices(
        &self, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_billing_invoices(limit, offset)
            .await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_customer_portal(&self, json: &str) -> Result<String, String> {
        let req: CustomerPortalRequest = serde_json::from_str(json).map_err(agentsmesh_services::wire)?;
        let resp = self.client.get_customer_portal(&req).await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_deployment_info(&self) -> Result<String, String> {
        let resp = self.client.get_billing_deployment_info().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_public_pricing(&self) -> Result<String, String> {
        let resp = self.client.get_public_pricing().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    pub async fn get_public_deployment_info(&self) -> Result<String, String> {
        let resp = self.client.get_public_deployment_info().await.map_err(agentsmesh_services::wire)?;
        serde_json::to_string(&resp).map_err(agentsmesh_services::wire)
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes (Uint8Array on the JS
    // side) and returns prost-encoded bytes — TS callers encode via
    // @bufbuild/protobuf .toBinary() and decode via .fromBinary().

    pub async fn get_overview_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_overview_connect(request_bytes)
            .await
    }

    pub async fn list_plans_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .list_plans_connect(request_bytes)
            .await
    }

    pub async fn get_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_subscription_connect(request_bytes)
            .await
    }

    pub async fn create_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .create_subscription_connect(request_bytes)
            .await
    }

    pub async fn update_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .update_subscription_connect(request_bytes)
            .await
    }

    pub async fn cancel_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .cancel_subscription_connect(request_bytes)
            .await
    }

    pub async fn request_cancel_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .request_cancel_connect(request_bytes)
            .await
    }

    pub async fn reactivate_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .reactivate_connect(request_bytes)
            .await
    }

    pub async fn upgrade_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .upgrade_connect(request_bytes)
            .await
    }

    pub async fn change_cycle_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .change_cycle_connect(request_bytes)
            .await
    }

    pub async fn update_auto_renew_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .update_auto_renew_connect(request_bytes)
            .await
    }

    pub async fn get_seat_usage_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_seat_usage_connect(request_bytes)
            .await
    }

    pub async fn purchase_seats_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .purchase_seats_connect(request_bytes)
            .await
    }

    pub async fn list_invoices_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .list_invoices_connect(request_bytes)
            .await
    }

    pub async fn create_checkout_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .create_checkout_connect(request_bytes)
            .await
    }

    pub async fn get_checkout_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_checkout_status_connect(request_bytes)
            .await
    }

    pub async fn get_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_deployment_info_connect(request_bytes)
            .await
    }

    pub async fn get_public_pricing_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_public_pricing_connect(request_bytes)
            .await
    }

    pub async fn get_public_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        agentsmesh_services::BillingService::new(self.client.clone())
            .get_public_deployment_info_connect(request_bytes)
            .await
    }
}
