#[cfg(test)]
mod api_channel_extension_tests {
    use std::sync::{Arc, Mutex};

    use serde_json::json;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{ApiClient, AuthTokenStore};

    struct MockTokenStore {
        org_slug: Mutex<Option<String>>,
    }
    impl MockTokenStore {
        fn with_org(slug: &str) -> Arc<Self> {
            Arc::new(Self { org_slug: Mutex::new(Some(slug.into())) })
        }
        fn no_org() -> Arc<Self> {
            Arc::new(Self { org_slug: Mutex::new(None) })
        }
    }
    impl AuthTokenStore for MockTokenStore {
        fn get_token(&self) -> Option<String> { Some("tok".into()) }
        fn get_refresh_token(&self) -> Option<String> { None }
        fn set_tokens(&self, _t: String, _r: String, _e: Option<i64>) {}
        fn clear_tokens(&self) {}
        fn get_current_org_slug(&self) -> Option<String> {
            self.org_slug.lock().unwrap().clone()
        }
    }

    fn ok(body: serde_json::Value) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(body)
    }

    // ── invitation ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_invitation() {
        let s = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/orgs/acme/invitations"))
            .and(body_json(json!({"email":"a@b.com","role":"member"})))
            .respond_with(ok(json!({
                "id":1,"email":"a@b.com","role":"member"
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let data = agentsmesh_types::CreateInvitationRequest {
            email: "a@b.com".into(),
            role: "member".into(),
        };
        let r = c.create_invitation(&data).await.unwrap();
        assert_eq!(r.email, "a@b.com");
    }

    #[tokio::test]
    async fn revoke_invitation() {
        let s = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/acme/invitations/9"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.revoke_invitation(9).await.unwrap();
    }

    #[tokio::test]
    async fn resend_invitation() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/acme/invitations/9/resend"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.resend_invitation(9).await.unwrap();
    }

    #[tokio::test]
    async fn accept_invitation() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/invitations/tok-xyz/accept"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let _ = c.accept_invitation("tok-xyz").await.unwrap();
        let reqs = s.received_requests().await.unwrap();
        assert!(reqs[0].headers.get("Authorization").is_none());
    }

    #[tokio::test]
    async fn list_pending_invitations() {
        let s = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/invitations/pending"))
            .respond_with(ok(json!({"invitations":[]})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let r = c.list_pending_invitations().await.unwrap();
        assert!(r.invitations.is_empty());
    }

    // ── loop_api ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn list_loops_no_filter() {
        let s = MockServer::start().await;
        Mock::given(method("GET")).and(path("/api/v1/orgs/acme/loops"))
            .respond_with(ok(json!({"loops":[]})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c.list_loops(None, None, None).await.unwrap();
        assert!(r.loops.is_empty());
    }

    #[tokio::test]
    async fn list_loops_with_pagination() {
        let s = MockServer::start().await;
        Mock::given(method("GET")).and(path("/api/v1/orgs/acme/loops"))
            .and(query_param("limit", "10"))
            .and(query_param("offset", "20"))
            .respond_with(ok(json!({"loops":[],"total":50})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c.list_loops(None, Some(10), Some(20)).await.unwrap();
        assert_eq!(r.total, Some(50));
    }

    #[tokio::test]
    async fn get_loop() {
        let s = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/acme/loops/daily-check"))
            .respond_with(ok(json!({
                "slug":"daily-check","name":"Daily Check","is_enabled":true
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c.get_loop("daily-check").await.unwrap();
        assert_eq!(r.slug, "daily-check");
        assert!(r.is_enabled);
    }

    #[tokio::test]
    async fn create_loop() {
        let s = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/orgs/acme/loops"))
            .and(body_json(json!({
                "name":"Nightly","slug":null,"description":null,
                "agent_slug":null,"custom_agent_slug":null,
                "permission_mode":null,"prompt_template":null,
                "prompt_variables":null,"repository_id":null,
                "runner_id":null,"branch_name":null,"ticket_id":null,
                "credential_profile_id":null,"config_overrides":null,
                "execution_mode":null,"cron_expression":null,
                "autopilot_config":null,"callback_url":null,
                "sandbox_strategy":null,"session_persistence":null,
                "concurrency_policy":null,"max_concurrent_runs":null,
                "max_retained_runs":null,"timeout_minutes":null
            })))
            .respond_with(ok(json!({
                "slug":"nightly","name":"Nightly","is_enabled":false
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let data = agentsmesh_types::CreateLoopRequest {
            name: "Nightly".into(),
            slug: None,
            description: None,
            agent_slug: None,
            custom_agent_slug: None,
            permission_mode: None,
            prompt_template: None,
            prompt_variables: None,
            repository_id: None,
            runner_id: None,
            branch_name: None,
            ticket_id: None,
            credential_profile_id: None,
            config_overrides: None,
            execution_mode: None,
            cron_expression: None,
            autopilot_config: None,
            callback_url: None,
            sandbox_strategy: None,
            session_persistence: None,
            concurrency_policy: None,
            max_concurrent_runs: None,
            max_retained_runs: None,
            timeout_minutes: None,
        };
        let r = c.create_loop(&data).await.unwrap();
        assert_eq!(r.name, "Nightly");
    }

    #[tokio::test]
    async fn update_loop() {
        let s = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/acme/loops/nightly"))
            .and(body_json(json!({
                "name":"Nightly v2","description":null,
                "agent_slug":null,"prompt_template":null,
                "prompt_variables":null,"repository_id":null,
                "runner_id":null,"branch_name":null,
                "cron_expression":null,"autopilot_config":null,
                "sandbox_strategy":null,"session_persistence":null,
                "concurrency_policy":null,"max_concurrent_runs":null,
                "max_retained_runs":null,"timeout_minutes":null
            })))
            .respond_with(ok(json!({
                "slug":"nightly","name":"Nightly v2","is_enabled":true
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let data = agentsmesh_types::UpdateLoopRequest {
            name: Some("Nightly v2".into()),
            description: None,
            agent_slug: None,
            prompt_template: None,
            prompt_variables: None,
            repository_id: None,
            runner_id: None,
            branch_name: None,
            cron_expression: None,
            autopilot_config: None,
            sandbox_strategy: None,
            session_persistence: None,
            concurrency_policy: None,
            max_concurrent_runs: None,
            max_retained_runs: None,
            timeout_minutes: None,
        };
        let r = c.update_loop("nightly", &data).await.unwrap();
        assert_eq!(r.name, "Nightly v2");
    }

    #[tokio::test]
    async fn delete_loop() {
        let s = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/acme/loops/old-loop"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.delete_loop("old-loop").await.unwrap();
    }

    #[tokio::test]
    async fn enable_loop() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/enable"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.enable_loop("my-loop").await.unwrap();
    }

    #[tokio::test]
    async fn disable_loop() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/disable"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.disable_loop("my-loop").await.unwrap();
    }

    #[tokio::test]
    async fn trigger_loop() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/trigger"))
            .respond_with(ok(json!({
                "id":1,"loop_slug":"my-loop","status":"running"
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c.trigger_loop("my-loop").await.unwrap();
        assert_eq!(r.status, agentsmesh_types::LoopRunStatus::Running);
    }

    #[tokio::test]
    async fn list_loop_runs() {
        let s = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/runs"))
            .and(query_param("status", "completed"))
            .respond_with(ok(json!({"runs":[],"total":0})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c
            .list_loop_runs("my-loop", Some("completed"), None, None)
            .await
            .unwrap();
        assert!(r.runs.is_empty());
    }

    #[tokio::test]
    async fn get_loop_run() {
        let s = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/runs/7"))
            .respond_with(ok(json!({
                "id":7,"loop_slug":"my-loop","status":"completed"
            })))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let r = c.get_loop_run("my-loop", 7).await.unwrap();
        assert_eq!(r.id, 7);
        assert_eq!(r.status, agentsmesh_types::LoopRunStatus::Completed);
    }

    #[tokio::test]
    async fn cancel_loop_run() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/acme/loops/my-loop/runs/7/cancel"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.cancel_loop_run("my-loop", 7).await.unwrap();
    }
}
