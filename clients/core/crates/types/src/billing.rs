use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingOverview {
    pub subscription: Option<Subscription>,
    pub usage_summary: Option<serde_json::Value>,
    pub seats: Option<SeatUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub plan_name: Option<String>,
    pub status: Option<String>,
    pub billing_cycle: Option<String>,
    pub current_period_start: Option<String>,
    pub current_period_end: Option<String>,
    pub auto_renew: Option<bool>,
    pub seats: Option<i64>,
    pub cancel_at_period_end: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub features: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanPrice {
    pub plan_name: String,
    pub currency: String,
    pub monthly_price: Option<f64>,
    pub annual_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingUsage {
    pub usage_type: Option<String>,
    pub current: Option<f64>,
    pub limit: Option<f64>,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatUsage {
    pub total: Option<i64>,
    pub used: Option<i64>,
    pub available: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub invoice_url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutStatus {
    pub order_no: String,
    pub status: Option<String>,
    pub payment_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub billing_enabled: Option<bool>,
    pub payment_providers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    pub plans: Option<Vec<Plan>>,
    pub currencies: Option<Vec<String>>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortalRequest {
    pub return_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortalResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanListResponse {
    pub plans: Vec<Plan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanPriceListResponse {
    pub prices: Vec<PlanPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceListResponse {
    pub invoices: Vec<Invoice>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaCheckResponse {
    pub allowed: bool,
    pub current: Option<f64>,
    pub limit: Option<f64>,
}
