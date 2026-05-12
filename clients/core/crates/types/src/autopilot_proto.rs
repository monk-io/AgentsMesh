// Hand-maintained `prost::Message` mirrors of `proto/autopilot/v1/autopilot.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// catches drift. NO `Serialize` / `Deserialize` derives (conventions §2.5).

#[derive(Clone, PartialEq, prost::Message)]
pub struct CircuitBreaker {
    #[prost(string, tag = "1")]
    pub state: String,
    #[prost(string, tag = "2")]
    pub reason: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AutopilotController {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub autopilot_controller_key: String,
    #[prost(string, tag = "3")]
    pub pod_key: String,
    #[prost(string, tag = "4")]
    pub phase: String,
    #[prost(int32, tag = "5")]
    pub current_iteration: i32,
    #[prost(int32, tag = "6")]
    pub max_iterations: i32,
    #[prost(message, optional, tag = "7")]
    pub circuit_breaker: Option<CircuitBreaker>,
    #[prost(bool, tag = "8")]
    pub user_takeover: bool,
    #[prost(string, tag = "9")]
    pub prompt: String,
    #[prost(string, optional, tag = "10")]
    pub started_at: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub last_iteration_at: Option<String>,
    #[prost(string, tag = "12")]
    pub created_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AutopilotIteration {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub controller_key: String,
    #[prost(int64, tag = "3")]
    pub iteration_number: i64,
    #[prost(string, tag = "4")]
    pub status: String,
    #[prost(string, tag = "5")]
    pub result: String,
    #[prost(string, optional, tag = "6")]
    pub started_at: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub completed_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAutopilotControllersRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAutopilotControllersResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<AutopilotController>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAutopilotControllerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateAutopilotControllerRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub pod_key: String,
    #[prost(string, tag = "3")]
    pub prompt: String,
    #[prost(int32, tag = "4")]
    pub max_iterations: i32,
    #[prost(int32, tag = "5")]
    pub iteration_timeout_sec: i32,
    #[prost(int32, tag = "6")]
    pub no_progress_threshold: i32,
    #[prost(int32, tag = "7")]
    pub same_error_threshold: i32,
    #[prost(int32, tag = "8")]
    pub approval_timeout_min: i32,
    #[prost(string, tag = "9")]
    pub control_agent_slug: String,
    #[prost(string, tag = "10")]
    pub control_prompt_template: String,
    #[prost(string, tag = "11")]
    pub mcp_config_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ActionRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ApproveRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub key: String,
    #[prost(bool, optional, tag = "3")]
    pub continue_execution: Option<bool>,
    #[prost(int32, tag = "4")]
    pub additional_iterations: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ActionResponse {
    #[prost(string, tag = "1")]
    pub status: String,
    #[prost(string, tag = "2")]
    pub action: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetIterationsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub key: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetIterationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<AutopilotIteration>,
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

    #[test]
    fn autopilot_controller_round_trip() {
        let original = AutopilotController {
            id: 42,
            autopilot_controller_key: "autopilot-abc".into(),
            pod_key: "pod-xyz".into(),
            phase: "running".into(),
            current_iteration: 3,
            max_iterations: 10,
            circuit_breaker: Some(CircuitBreaker {
                state: "closed".into(),
                reason: "".into(),
            }),
            user_takeover: false,
            prompt: "fix the bug".into(),
            started_at: Some("2026-05-12T13:16:10Z".into()),
            last_iteration_at: Some("2026-05-12T13:20:00Z".into()),
            created_at: "2026-05-12T13:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        assert_eq!(original, AutopilotController::decode(&*bytes).unwrap());
    }

    #[test]
    fn create_request_round_trip() {
        let req = CreateAutopilotControllerRequest {
            org_slug: "acme".into(),
            pod_key: "pod-1".into(),
            prompt: "fix it".into(),
            max_iterations: 5,
            iteration_timeout_sec: 0,
            no_progress_threshold: 0,
            same_error_threshold: 0,
            approval_timeout_min: 0,
            control_agent_slug: "".into(),
            control_prompt_template: "".into(),
            mcp_config_json: "".into(),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, CreateAutopilotControllerRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn approve_request_optional_continue_round_trip() {
        let with_true = ApproveRequest {
            org_slug: "acme".into(),
            key: "ap-1".into(),
            continue_execution: Some(true),
            additional_iterations: 5,
        };
        let absent = ApproveRequest {
            continue_execution: None,
            ..with_true.clone()
        };
        assert_ne!(with_true.encode_to_vec(), absent.encode_to_vec());
        let r1 = ApproveRequest::decode(&*with_true.encode_to_vec()).unwrap();
        let r2 = ApproveRequest::decode(&*absent.encode_to_vec()).unwrap();
        assert_eq!(r1.continue_execution, Some(true));
        assert_eq!(r2.continue_execution, None);
    }

    #[test]
    fn list_response_round_trip() {
        let resp = ListAutopilotControllersResponse {
            items: vec![],
            total: 0,
            limit: 20,
            offset: 0,
        };
        let bytes = resp.encode_to_vec();
        assert_eq!(resp, ListAutopilotControllersResponse::decode(&*bytes).unwrap());
    }
}
