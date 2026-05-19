use serde::{Deserialize, Serialize};

// JSON-bridge request shapes used by `services/src/billing.rs` to decode the
// legacy `Service::*(json: &str)` entry points before forwarding to Connect.
// The wasm renderer is fully on Connect; once the dual-track JSON entry points
// are retired these types vanish with them.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_name: String,
    pub billing_cycle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub plan_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub order_type: Option<String>,
    pub plan_name: String,
    pub billing_cycle: Option<String>,
    pub seats: Option<i64>,
    pub provider: Option<String>,
    pub success_url: Option<String>,
    pub cancel_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSubscriptionRequest {
    pub immediate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeSubscriptionRequest {
    pub plan_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBillingCycleRequest {
    pub billing_cycle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutoRenewRequest {
    pub auto_renew: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSeatsRequest {
    pub seats: i64,
}
