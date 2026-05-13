// Hand-maintained `prost::Message` mirrors of
// `proto/license/v1/license.proto`. Tag numbers match the .proto
// byte-for-byte; `tools/validate_prost_tags` runs at build time to catch
// drift (watch list §8). NO `Serialize`/`Deserialize` derives on these
// structs — binary wire only (conventions §2.5, §3).
//
// Two services share this module — `LicenseService` (auth-required
// mutations + validation) and `LicensePublicService` (read-only status /
// limits / feature checks). The wire shape is the same for both: every
// request is its own message type so they can evolve independently.

// --- Status / limits / preview entities ---

#[derive(Clone, PartialEq, prost::Message)]
pub struct LicenseStatus {
    #[prost(bool, tag = "1")]
    pub is_active: bool,
    #[prost(string, tag = "2")]
    pub license_key: String,
    #[prost(string, tag = "3")]
    pub organization_name: String,
    #[prost(string, tag = "4")]
    pub plan: String,
    #[prost(string, optional, tag = "5")]
    pub expires_at: Option<String>,
    #[prost(int32, tag = "6")]
    pub max_users: i32,
    #[prost(int32, tag = "7")]
    pub max_runners: i32,
    #[prost(int32, tag = "8")]
    pub max_repositories: i32,
    #[prost(int32, tag = "9")]
    pub max_pod_minutes: i32,
    #[prost(string, repeated, tag = "10")]
    pub features: Vec<String>,
    #[prost(string, tag = "11")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ValidatedLicense {
    #[prost(bool, tag = "1")]
    pub valid: bool,
    #[prost(string, tag = "2")]
    pub license_key: String,
    #[prost(string, tag = "3")]
    pub organization_name: String,
    #[prost(string, tag = "4")]
    pub contact_email: String,
    #[prost(string, tag = "5")]
    pub plan: String,
    #[prost(message, optional, tag = "6")]
    pub limits: Option<LicenseLimits>,
    #[prost(string, repeated, tag = "7")]
    pub features: Vec<String>,
    #[prost(string, tag = "8")]
    pub issued_at: String,
    #[prost(string, tag = "9")]
    pub expires_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct LicenseLimits {
    #[prost(int32, tag = "1")]
    pub max_users: i32,
    #[prost(int32, tag = "2")]
    pub max_runners: i32,
    #[prost(int32, tag = "3")]
    pub max_repositories: i32,
    #[prost(int32, tag = "4")]
    pub max_pod_minutes: i32,
}

// --- Requests ---

#[derive(Clone, PartialEq, prost::Message)]
pub struct ActivateLicenseRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub license_data: Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RefreshLicenseRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ValidateLicenseRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub license_data: Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetLicenseStatusRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetLicenseLimitsRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CheckFeatureRequest {
    #[prost(string, tag = "1")]
    pub feature: String,
}

// --- Responses ---

#[derive(Clone, PartialEq, prost::Message)]
pub struct LicenseLimitsResponse {
    #[prost(message, optional, tag = "1")]
    pub limits: Option<LicenseLimits>,
    #[prost(string, tag = "2")]
    pub plan: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CheckFeatureResponse {
    #[prost(string, tag = "1")]
    pub feature: String,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_limits() -> LicenseLimits {
        LicenseLimits {
            max_users: 50,
            max_runners: 10,
            max_repositories: 100,
            max_pod_minutes: -1,
        }
    }

    fn sample_status() -> LicenseStatus {
        LicenseStatus {
            is_active: true,
            license_key: "LK-2026-ABCDEF".into(),
            organization_name: "Acme Corp".into(),
            plan: "enterprise".into(),
            expires_at: Some("2027-05-13T00:00:00Z".into()),
            max_users: 50,
            max_runners: 10,
            max_repositories: 100,
            max_pod_minutes: -1,
            features: vec!["sso".into(), "audit_logs".into()],
            message: "License is active".into(),
        }
    }

    #[test]
    fn license_status_round_trip_preserves_every_field() {
        let original = sample_status();
        let bytes = original.encode_to_vec();
        let decoded = LicenseStatus::decode(&*bytes).unwrap();
        assert_eq!(
            original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here"
        );
    }

    #[test]
    fn license_status_inactive_with_no_expiry() {
        let original = LicenseStatus {
            is_active: false,
            message: "No license installed".into(),
            // Every other scalar defaults; expires_at None.
            ..Default::default()
        };
        let bytes = original.encode_to_vec();
        let decoded = LicenseStatus::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.expires_at, None);
    }

    #[test]
    fn validated_license_round_trip_preserves_nested_limits() {
        let original = ValidatedLicense {
            valid: true,
            license_key: "LK-PREVIEW".into(),
            organization_name: "Preview Org".into(),
            contact_email: "ops@preview.example".into(),
            plan: "team".into(),
            limits: Some(sample_limits()),
            features: vec!["sso".into()],
            issued_at: "2026-05-13T00:00:00Z".into(),
            expires_at: "2027-05-13T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = ValidatedLicense::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.limits.unwrap().max_users, 50);
    }

    #[test]
    fn license_limits_unlimited_encoding() {
        let original = LicenseLimits {
            max_users: -1,
            max_runners: -1,
            max_repositories: -1,
            max_pod_minutes: -1,
        };
        let bytes = original.encode_to_vec();
        let decoded = LicenseLimits::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn activate_license_request_carries_raw_bytes() {
        let blob = b"{\"license_key\":\"foo\",\"signature\":\"abc\"}".to_vec();
        let original = ActivateLicenseRequest {
            license_data: blob.clone(),
        };
        let bytes = original.encode_to_vec();
        let decoded = ActivateLicenseRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.license_data, blob);
    }

    #[test]
    fn empty_requests_encode_to_zero_bytes() {
        // Proto3 default field elision: empty messages encode to no bytes
        // on the wire. Asserting this so a future schema change to add a
        // required field surfaces as a test break.
        assert!(RefreshLicenseRequest {}.encode_to_vec().is_empty());
        assert!(GetLicenseStatusRequest {}.encode_to_vec().is_empty());
        assert!(GetLicenseLimitsRequest {}.encode_to_vec().is_empty());
    }

    #[test]
    fn check_feature_round_trip() {
        let original = CheckFeatureRequest {
            feature: "sso".into(),
        };
        let bytes = original.encode_to_vec();
        assert_eq!(original, CheckFeatureRequest::decode(&*bytes).unwrap());

        let resp = CheckFeatureResponse {
            feature: "sso".into(),
            enabled: true,
        };
        let resp_bytes = resp.encode_to_vec();
        assert_eq!(resp, CheckFeatureResponse::decode(&*resp_bytes).unwrap());
    }

    #[test]
    fn license_limits_response_round_trip() {
        let original = LicenseLimitsResponse {
            limits: Some(sample_limits()),
            plan: "enterprise".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = LicenseLimitsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
