use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePromoRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedeemPromoRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoValidationResult {
    pub valid: bool,
    pub discount_type: Option<String>,
    pub discount_value: Option<f64>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodeHistory {
    pub code: String,
    pub redeemed_at: Option<String>,
    pub discount_type: Option<String>,
    pub discount_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodeHistoryResponse {
    pub history: Vec<PromoCodeHistory>,
}
