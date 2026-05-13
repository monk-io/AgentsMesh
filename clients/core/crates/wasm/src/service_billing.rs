use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
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

    // Legacy JSON-shaped methods kept as thin wrappers over the Rust
    // `BillingService` (which talks Connect-RPC under the hood). The web
    // renderer has migrated to the `*_connect` binary methods, but desktop
    // node-bridge + iOS FFI still call these to preserve the legacy JSON
    // wire shape across the wasm/NAPI boundary.

    pub async fn get_overview(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_overview().await
    }

    pub async fn get_subscription(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_subscription().await
    }

    pub async fn create_subscription(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).create_subscription(json).await
    }

    pub async fn cancel_subscription(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).cancel_subscription().await
    }

    pub async fn update_subscription(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).update_subscription(json).await
    }

    pub async fn list_plans(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).list_plans().await
    }

    pub async fn get_usage(&self, usage_type: Option<String>) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_usage(usage_type).await
    }

    pub async fn check_quota(&self, resource: &str, amount: Option<u32>) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).check_quota(resource, amount).await
    }

    pub async fn create_checkout(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).create_checkout(json).await
    }

    pub async fn get_checkout_status(&self, order_no: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_checkout_status(order_no).await
    }

    pub async fn request_cancel(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).request_cancel(json).await
    }

    pub async fn reactivate(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).reactivate().await
    }

    pub async fn upgrade(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).upgrade(json).await
    }

    pub async fn change_cycle(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).change_cycle(json).await
    }

    pub async fn update_auto_renew(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).update_auto_renew(json).await
    }

    pub async fn get_seat_usage(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_seat_usage().await
    }

    pub async fn purchase_seats(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).purchase_seats(json).await
    }

    pub async fn list_invoices(
        &self, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).list_invoices(limit, offset).await
    }

    pub async fn get_customer_portal(&self, json: &str) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_customer_portal(json).await
    }

    pub async fn get_deployment_info(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_deployment_info().await
    }

    pub async fn get_public_pricing(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_public_pricing().await
    }

    pub async fn get_public_deployment_info(&self) -> Result<String, String> {
        agentsmesh_services::BillingService::new(self.client.clone()).get_public_deployment_info().await
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
