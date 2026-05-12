// Hand-maintained `prost::Message` mirrors of `proto/promocode/v1/promocode.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift (watch list §8). NO `Serialize` /
// `Deserialize` derives — binary wire only (conventions §2.5, §3).
//
// Org-scoped surface only (Validate / Redeem / GetRedemptionHistory). The
// admin CRUD over promo codes stays on REST during this migration; those
// endpoints are gated by ADMIN middleware, not org middleware, so they
// migrate as part of the admin/ surface sweep.

// ----- Entities -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct Redemption {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub promo_code_id: i64,
    #[prost(int64, tag = "3")]
    pub organization_id: i64,
    #[prost(int64, tag = "4")]
    pub user_id: i64,
    #[prost(string, tag = "5")]
    pub plan_name: String,
    #[prost(int32, tag = "6")]
    pub duration_months: i32,
    #[prost(string, optional, tag = "7")]
    pub previous_plan_name: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub previous_period_end: Option<String>,
    #[prost(string, tag = "9")]
    pub new_period_end: String,
    #[prost(string, optional, tag = "10")]
    pub ip_address: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub user_agent: Option<String>,
    #[prost(string, tag = "12")]
    pub created_at: String,
}

// ----- Validate -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct ValidatePromoCodeRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub code: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ValidatePromoCodeResponse {
    #[prost(bool, tag = "1")]
    pub valid: bool,
    #[prost(string, tag = "2")]
    pub code: String,
    #[prost(string, optional, tag = "3")]
    pub plan_name: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub plan_display_name: Option<String>,
    #[prost(int32, optional, tag = "5")]
    pub duration_months: Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub expires_at: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub message_code: Option<String>,
}

// ----- Redeem -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct RedeemPromoCodeRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub code: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RedeemPromoCodeResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, optional, tag = "2")]
    pub plan_name: Option<String>,
    #[prost(int32, optional, tag = "3")]
    pub duration_months: Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub new_period_end: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub message_code: Option<String>,
}

// ----- GetRedemptionHistory -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRedemptionHistoryRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetRedemptionHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Redemption>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_redemption() -> Redemption {
        Redemption {
            id: 1,
            promo_code_id: 7,
            organization_id: 42,
            user_id: 100,
            plan_name: "pro".into(),
            duration_months: 3,
            previous_plan_name: Some("free".into()),
            previous_period_end: Some("2026-05-01T00:00:00Z".into()),
            new_period_end: "2026-08-01T00:00:00Z".into(),
            ip_address: Some("203.0.113.42".into()),
            user_agent: Some("Mozilla/5.0".into()),
            created_at: "2026-05-12T13:16:10Z".into(),
        }
    }

    #[test]
    fn redemption_round_trip_preserves_every_field() {
        let original = sample_redemption();
        let bytes = original.encode_to_vec();
        let decoded = Redemption::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn redemption_with_no_previous_plan_round_trips() {
        let first_time = Redemption {
            previous_plan_name: None,
            previous_period_end: None,
            ip_address: None,
            user_agent: None,
            ..sample_redemption()
        };
        let bytes = first_time.encode_to_vec();
        let decoded = Redemption::decode(&*bytes).unwrap();
        assert_eq!(first_time, decoded);
        assert!(decoded.previous_plan_name.is_none());
    }

    #[test]
    fn validate_request_round_trip() {
        let original = ValidatePromoCodeRequest {
            org_slug: "acme".into(),
            code: "WELCOME2026".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = ValidatePromoCodeRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn validate_response_valid_path_round_trips() {
        let valid = ValidatePromoCodeResponse {
            valid: true,
            code: "WELCOME2026".into(),
            plan_name: Some("pro".into()),
            plan_display_name: Some("Pro".into()),
            duration_months: Some(3),
            expires_at: Some("2026-12-31T23:59:59Z".into()),
            message_code: None,
        };
        let bytes = valid.encode_to_vec();
        let decoded = ValidatePromoCodeResponse::decode(&*bytes).unwrap();
        assert_eq!(valid, decoded);
        assert!(decoded.valid);
        assert!(decoded.message_code.is_none());
    }

    #[test]
    fn validate_response_invalid_path_round_trips() {
        let invalid = ValidatePromoCodeResponse {
            valid: false,
            code: "BOGUS".into(),
            plan_name: None,
            plan_display_name: None,
            duration_months: None,
            expires_at: None,
            message_code: Some("promo_code_not_found".into()),
        };
        let bytes = invalid.encode_to_vec();
        let decoded = ValidatePromoCodeResponse::decode(&*bytes).unwrap();
        assert_eq!(invalid, decoded);
        assert!(!decoded.valid);
        assert_eq!(decoded.message_code.as_deref(), Some("promo_code_not_found"));
    }

    #[test]
    fn redeem_request_round_trip() {
        let req = RedeemPromoCodeRequest {
            org_slug: "acme".into(),
            code: "WELCOME2026".into(),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, RedeemPromoCodeRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn redeem_response_success_round_trips() {
        let success = RedeemPromoCodeResponse {
            success: true,
            plan_name: Some("pro".into()),
            duration_months: Some(3),
            new_period_end: Some("2026-08-01T00:00:00Z".into()),
            message_code: Some("promo_code_redeem_success".into()),
        };
        let bytes = success.encode_to_vec();
        let decoded = RedeemPromoCodeResponse::decode(&*bytes).unwrap();
        assert_eq!(success, decoded);
        assert!(decoded.success);
    }

    #[test]
    fn redeem_response_failure_round_trips() {
        let failure = RedeemPromoCodeResponse {
            success: false,
            plan_name: None,
            duration_months: None,
            new_period_end: None,
            message_code: Some("promo_code_not_owner".into()),
        };
        let bytes = failure.encode_to_vec();
        let decoded = RedeemPromoCodeResponse::decode(&*bytes).unwrap();
        assert_eq!(failure, decoded);
        assert!(!decoded.success);
        assert_eq!(decoded.message_code.as_deref(), Some("promo_code_not_owner"));
    }

    #[test]
    fn get_history_request_round_trip() {
        let req = GetRedemptionHistoryRequest {
            org_slug: "acme".into(),
            offset: Some(10),
            limit: Some(50),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, GetRedemptionHistoryRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn get_history_optional_offset_zero_distinguishable_from_absent() {
        let with_zero = GetRedemptionHistoryRequest {
            org_slug: "acme".into(),
            offset: Some(0),
            limit: None,
        };
        let absent = GetRedemptionHistoryRequest {
            org_slug: "acme".into(),
            offset: None,
            limit: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
        let r1 = GetRedemptionHistoryRequest::decode(&*with_zero.encode_to_vec()).unwrap();
        let r2 = GetRedemptionHistoryRequest::decode(&*absent.encode_to_vec()).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn get_history_response_round_trip() {
        let resp = GetRedemptionHistoryResponse {
            items: vec![sample_redemption()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = resp.encode_to_vec();
        let decoded = GetRedemptionHistoryResponse::decode(&*bytes).unwrap();
        assert_eq!(resp, decoded);
        assert_eq!(decoded.items.len(), 1);
    }

    #[test]
    fn get_history_empty_response_round_trips() {
        let empty = GetRedemptionHistoryResponse {
            items: vec![],
            total: 0,
            limit: 20,
            offset: 0,
        };
        let bytes = empty.encode_to_vec();
        let decoded = GetRedemptionHistoryResponse::decode(&*bytes).unwrap();
        assert_eq!(empty, decoded);
        assert!(decoded.items.is_empty());
    }
}
