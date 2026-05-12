use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use agentsmesh_types::proto_billing_v1 as billing_proto;
use prost::Message;

pub struct BillingService {
    client: Arc<ApiClient>,
}

impl BillingService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_overview(&self) -> Result<String, String> {
        let resp = self.client.get_billing_overview().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_subscription(&self) -> Result<String, String> {
        let resp = self.client.get_billing_subscription().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_subscription(&self, json: &str) -> Result<String, String> {
        let req: CreateSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.create_billing_subscription(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn cancel_subscription(&self) -> Result<String, String> {
        let resp = self.client.cancel_billing_subscription().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_subscription(&self, json: &str) -> Result<String, String> {
        let req: UpdateSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.update_billing_subscription(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn list_plans(&self) -> Result<String, String> {
        let resp = self.client.list_billing_plans().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_usage(&self, usage_type: Option<String>) -> Result<String, String> {
        let resp = self.client
            .get_billing_usage(usage_type.as_deref())
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn check_quota(&self, resource: &str, amount: Option<u32>) -> Result<String, String> {
        let resp = self.client
            .check_billing_quota(resource, amount)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_checkout(&self, json: &str) -> Result<String, String> {
        let req: CheckoutRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.create_billing_checkout(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_checkout_status(&self, order_no: &str) -> Result<String, String> {
        let resp = self.client
            .get_billing_checkout_status(order_no)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn request_cancel(&self, json: &str) -> Result<String, String> {
        let req: CancelSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.request_cancel_subscription(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn reactivate(&self) -> Result<String, String> {
        let resp = self.client.reactivate_subscription().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn upgrade(&self, json: &str) -> Result<String, String> {
        let req: UpgradeSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.upgrade_subscription(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn change_cycle(&self, json: &str) -> Result<String, String> {
        let req: ChangeBillingCycleRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.change_billing_cycle(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn update_auto_renew(&self, json: &str) -> Result<String, String> {
        let req: UpdateAutoRenewRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.update_auto_renew(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_seat_usage(&self) -> Result<String, String> {
        let resp = self.client.get_seat_usage().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn purchase_seats(&self, json: &str) -> Result<String, String> {
        let req: PurchaseSeatsRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.purchase_seats(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn list_invoices(
        &self, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let resp = self.client
            .list_billing_invoices(limit, offset)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_customer_portal(&self, json: &str) -> Result<String, String> {
        let req: CustomerPortalRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.get_customer_portal(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_deployment_info(&self) -> Result<String, String> {
        let resp = self.client.get_billing_deployment_info().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_public_pricing(&self) -> Result<String, String> {
        let resp = self.client.get_public_pricing().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_public_deployment_info(&self) -> Result<String, String> {
        let resp = self.client.get_public_deployment_info().await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes and returns
    // prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
    // surface (conventions §2.5). Caller (TS) encodes via
    // @bufbuild/protobuf .toBinary() and decodes via .fromBinary().

    pub async fn get_overview_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetOverviewRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_overview request: {e}"))?;
        let resp = self.client.get_billing_overview_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_plans_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ListPlansRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_plans request: {e}"))?;
        let resp = self.client.list_billing_plans_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_subscription request: {e}"))?;
        let resp = self.client.get_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CreateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_subscription request: {e}"))?;
        let resp = self.client.create_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpdateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_subscription request: {e}"))?;
        let resp = self.client.update_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn cancel_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CancelSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode cancel_subscription request: {e}"))?;
        let resp = self.client.cancel_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn request_cancel_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::RequestCancelSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode request_cancel request: {e}"))?;
        let resp = self.client.request_cancel_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn reactivate_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ReactivateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode reactivate request: {e}"))?;
        let resp = self.client.reactivate_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn upgrade_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpgradeSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode upgrade request: {e}"))?;
        let resp = self.client.upgrade_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn change_cycle_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ChangeBillingCycleRequest::decode(request_bytes)
            .map_err(|e| format!("decode change_cycle request: {e}"))?;
        let resp = self.client.change_billing_cycle_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_auto_renew_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpdateAutoRenewRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_auto_renew request: {e}"))?;
        let resp = self.client.update_auto_renew_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_seat_usage_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetSeatUsageRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_seat_usage request: {e}"))?;
        let resp = self.client.get_seat_usage_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn purchase_seats_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::PurchaseSeatsRequest::decode(request_bytes)
            .map_err(|e| format!("decode purchase_seats request: {e}"))?;
        let resp = self.client.purchase_seats_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_invoices_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ListInvoicesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_invoices request: {e}"))?;
        let resp = self.client.list_billing_invoices_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_checkout_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CreateCheckoutRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_checkout request: {e}"))?;
        let resp = self.client.create_billing_checkout_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_checkout_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetCheckoutStatusRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_checkout_status request: {e}"))?;
        let resp = self.client.get_billing_checkout_status_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetDeploymentInfoRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_deployment_info request: {e}"))?;
        let resp = self.client.get_billing_deployment_info_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_public_pricing_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetPublicPricingRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_public_pricing request: {e}"))?;
        let resp = self.client.get_public_pricing_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_public_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetPublicDeploymentInfoRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_public_deployment_info request: {e}"))?;
        let resp = self.client.get_public_deployment_info_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
