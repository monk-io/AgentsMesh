// Hand-maintained `prost::Message` mirrors of `proto/loop/v1/loop.proto`.
// Tag numbers match the .proto byte-for-byte. NO `Serialize` / `Deserialize`
// derives (conventions §2.5, §3).

#[derive(Clone, PartialEq, prost::Message)]
pub struct Loop {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub slug: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(string, optional, tag = "4")]
    pub description: Option<String>,
    #[prost(string, tag = "5")]
    pub agent_slug: String,
    #[prost(string, tag = "6")]
    pub permission_mode: String,
    #[prost(string, tag = "7")]
    pub prompt_template: String,
    #[prost(string, tag = "8")]
    pub config_overrides_json: String,
    #[prost(string, tag = "9")]
    pub prompt_variables_json: String,
    #[prost(string, tag = "10")]
    pub execution_mode: String,
    #[prost(string, optional, tag = "11")]
    pub cron_expression: Option<String>,
    #[prost(string, tag = "12")]
    pub autopilot_config_json: String,
    #[prost(string, optional, tag = "13")]
    pub callback_url: Option<String>,
    #[prost(int64, optional, tag = "14")]
    pub repository_id: Option<i64>,
    #[prost(int64, optional, tag = "15")]
    pub runner_id: Option<i64>,
    #[prost(string, optional, tag = "16")]
    pub branch_name: Option<String>,
    #[prost(int64, optional, tag = "17")]
    pub ticket_id: Option<i64>,
    #[prost(int64, optional, tag = "18")]
    pub credential_profile_id: Option<i64>,
    #[prost(string, tag = "19")]
    pub status: String,
    #[prost(string, tag = "20")]
    pub sandbox_strategy: String,
    #[prost(bool, tag = "21")]
    pub session_persistence: bool,
    #[prost(string, tag = "22")]
    pub concurrency_policy: String,
    #[prost(int32, tag = "23")]
    pub max_concurrent_runs: i32,
    #[prost(int32, tag = "24")]
    pub max_retained_runs: i32,
    #[prost(int32, tag = "25")]
    pub timeout_minutes: i32,
    #[prost(int32, tag = "26")]
    pub idle_timeout_sec: i32,
    #[prost(int64, tag = "27")]
    pub total_runs: i64,
    #[prost(int64, tag = "28")]
    pub successful_runs: i64,
    #[prost(int64, tag = "29")]
    pub failed_runs: i64,
    #[prost(int64, tag = "30")]
    pub active_run_count: i64,
    #[prost(double, optional, tag = "31")]
    pub avg_duration_sec: Option<f64>,
    #[prost(string, optional, tag = "32")]
    pub last_run_at: Option<String>,
    #[prost(string, tag = "33")]
    pub created_at: String,
    #[prost(string, tag = "34")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct LoopRun {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub loop_id: i64,
    #[prost(int64, tag = "3")]
    pub run_number: i64,
    #[prost(string, tag = "4")]
    pub status: String,
    #[prost(string, optional, tag = "5")]
    pub pod_key: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub started_at: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub completed_at: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub error_message: Option<String>,
    #[prost(string, tag = "9")]
    pub created_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListLoopsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub status: String,
    #[prost(string, tag = "3")]
    pub execution_mode: String,
    #[prost(bool, optional, tag = "4")]
    pub cron_enabled: Option<bool>,
    #[prost(string, tag = "5")]
    pub query: String,
    #[prost(int32, optional, tag = "6")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListLoopsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<Loop>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetLoopRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateLoopRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub slug: String,
    #[prost(string, tag = "4")]
    pub description: String,
    #[prost(string, tag = "5")]
    pub agent_slug: String,
    #[prost(string, tag = "6")]
    pub permission_mode: String,
    #[prost(string, tag = "7")]
    pub prompt_template: String,
    #[prost(string, tag = "8")]
    pub prompt_variables_json: String,
    #[prost(string, tag = "9")]
    pub config_overrides_json: String,
    #[prost(string, tag = "10")]
    pub autopilot_config_json: String,
    #[prost(int64, optional, tag = "11")]
    pub repository_id: Option<i64>,
    #[prost(int64, optional, tag = "12")]
    pub runner_id: Option<i64>,
    #[prost(string, tag = "13")]
    pub branch_name: String,
    #[prost(int64, optional, tag = "14")]
    pub ticket_id: Option<i64>,
    #[prost(int64, optional, tag = "15")]
    pub credential_profile_id: Option<i64>,
    #[prost(string, tag = "16")]
    pub execution_mode: String,
    #[prost(string, tag = "17")]
    pub cron_expression: String,
    #[prost(string, tag = "18")]
    pub callback_url: String,
    #[prost(string, tag = "19")]
    pub sandbox_strategy: String,
    #[prost(bool, optional, tag = "20")]
    pub session_persistence: Option<bool>,
    #[prost(string, tag = "21")]
    pub concurrency_policy: String,
    #[prost(int32, optional, tag = "22")]
    pub max_concurrent_runs: Option<i32>,
    #[prost(int32, optional, tag = "23")]
    pub max_retained_runs: Option<i32>,
    #[prost(int32, optional, tag = "24")]
    pub timeout_minutes: Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub idle_timeout_sec: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateLoopRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub description: Option<String>,
    #[prost(string, tag = "5")]
    pub agent_slug: String,
    #[prost(string, optional, tag = "6")]
    pub permission_mode: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub prompt_template: Option<String>,
    #[prost(string, tag = "8")]
    pub prompt_variables_json: String,
    #[prost(string, tag = "9")]
    pub config_overrides_json: String,
    #[prost(string, tag = "10")]
    pub autopilot_config_json: String,
    #[prost(int64, optional, tag = "11")]
    pub repository_id: Option<i64>,
    #[prost(int64, optional, tag = "12")]
    pub runner_id: Option<i64>,
    #[prost(string, optional, tag = "13")]
    pub branch_name: Option<String>,
    #[prost(int64, optional, tag = "14")]
    pub ticket_id: Option<i64>,
    #[prost(int64, optional, tag = "15")]
    pub credential_profile_id: Option<i64>,
    #[prost(string, optional, tag = "16")]
    pub execution_mode: Option<String>,
    #[prost(string, optional, tag = "17")]
    pub cron_expression: Option<String>,
    #[prost(string, optional, tag = "18")]
    pub callback_url: Option<String>,
    #[prost(string, optional, tag = "19")]
    pub sandbox_strategy: Option<String>,
    #[prost(bool, optional, tag = "20")]
    pub session_persistence: Option<bool>,
    #[prost(string, optional, tag = "21")]
    pub concurrency_policy: Option<String>,
    #[prost(int32, optional, tag = "22")]
    pub max_concurrent_runs: Option<i32>,
    #[prost(int32, optional, tag = "23")]
    pub max_retained_runs: Option<i32>,
    #[prost(int32, optional, tag = "24")]
    pub timeout_minutes: Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub idle_timeout_sec: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteLoopRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteLoopResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct LoopActionRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TriggerLoopRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
    #[prost(string, tag = "3")]
    pub variables_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TriggerLoopResponse {
    #[prost(message, optional, tag = "1")]
    pub run: Option<LoopRun>,
    #[prost(bool, tag = "2")]
    pub skipped: bool,
    #[prost(string, tag = "3")]
    pub reason: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
    #[prost(string, tag = "3")]
    pub status: String,
    #[prost(int32, optional, tag = "4")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListRunsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<LoopRun>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CancelRunRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub loop_slug: String,
    #[prost(int64, tag = "3")]
    pub run_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CancelRunResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn loop_round_trip() {
        let original = Loop {
            id: 1,
            slug: "test-loop".into(),
            name: "Test Loop".into(),
            description: Some("a test".into()),
            agent_slug: "claude-code".into(),
            permission_mode: "default".into(),
            prompt_template: "do {{thing}}".into(),
            config_overrides_json: "{}".into(),
            prompt_variables_json: "{\"thing\":\"x\"}".into(),
            execution_mode: "autopilot".into(),
            cron_expression: Some("0 * * * *".into()),
            autopilot_config_json: "{}".into(),
            callback_url: None,
            repository_id: Some(42),
            runner_id: None,
            branch_name: None,
            ticket_id: None,
            credential_profile_id: None,
            status: "enabled".into(),
            sandbox_strategy: "persistent".into(),
            session_persistence: true,
            concurrency_policy: "skip".into(),
            max_concurrent_runs: 1,
            max_retained_runs: 100,
            timeout_minutes: 60,
            idle_timeout_sec: 30,
            total_runs: 5,
            successful_runs: 4,
            failed_runs: 1,
            active_run_count: 0,
            avg_duration_sec: Some(12.5),
            last_run_at: Some("2026-05-12T13:00:00Z".into()),
            created_at: "2026-04-01T00:00:00Z".into(),
            updated_at: "2026-05-12T13:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        assert_eq!(original, Loop::decode(&*bytes).unwrap());
    }

    #[test]
    fn list_loops_request_round_trip() {
        let req = ListLoopsRequest {
            org_slug: "acme".into(),
            status: "enabled".into(),
            execution_mode: "".into(),
            cron_enabled: Some(true),
            query: "".into(),
            offset: Some(0),
            limit: Some(20),
        };
        let bytes = req.encode_to_vec();
        assert_eq!(req, ListLoopsRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn trigger_skipped_round_trip() {
        let resp = TriggerLoopResponse {
            run: None,
            skipped: true,
            reason: "concurrency_policy=skip".into(),
        };
        let bytes = resp.encode_to_vec();
        let decoded = TriggerLoopResponse::decode(&*bytes).unwrap();
        assert!(decoded.skipped);
        assert_eq!(decoded.reason, "concurrency_policy=skip");
    }

    #[test]
    fn loop_run_round_trip() {
        let run = LoopRun {
            id: 1,
            loop_id: 42,
            run_number: 5,
            status: "running".into(),
            pod_key: Some("pod-1".into()),
            started_at: Some("2026-05-12T13:00:00Z".into()),
            completed_at: None,
            error_message: None,
            created_at: "2026-05-12T12:59:00Z".into(),
        };
        let bytes = run.encode_to_vec();
        assert_eq!(run, LoopRun::decode(&*bytes).unwrap());
    }
}
