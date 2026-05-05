use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageDashboard {
    pub total_input_tokens: Option<i64>,
    pub total_output_tokens: Option<i64>,
    pub total_cost: Option<f64>,
    pub data_points: Option<Vec<TokenUsageDataPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageDataPoint {
    pub timestamp: String,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub cost: Option<f64>,
    pub agent_slug: Option<String>,
    pub model: Option<String>,
}
