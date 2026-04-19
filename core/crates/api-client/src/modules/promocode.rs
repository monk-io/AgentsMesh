use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

impl ApiClient {
    pub async fn validate_promo_code(
        &self,
        data: &ValidatePromoRequest,
    ) -> Result<PromoValidationResult, ApiError> {
        self.post(&self.org_path("/billing/promo-codes/validate"), data)
            .await
    }

    pub async fn redeem_promo_code(
        &self,
        data: &RedeemPromoRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path("/billing/promo-codes/redeem"), data)
            .await
    }

    pub async fn get_promo_code_history(&self) -> Result<PromoCodeHistoryResponse, ApiError> {
        self.get(&self.org_path("/billing/promo-codes/history"))
            .await
    }
}
