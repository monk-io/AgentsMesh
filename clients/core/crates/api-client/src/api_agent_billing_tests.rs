#[cfg(test)]
mod api_agent_billing_tests {
    use std::sync::Mutex;

    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{ApiClient, AuthTokenStore};

    struct MockTokenStore {
        org_slug: Mutex<Option<String>>,
    }
    impl MockTokenStore {
        fn with_org(slug: &str) -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self { org_slug: Mutex::new(Some(slug.into())) })
        }
        fn no_org() -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self { org_slug: Mutex::new(None) })
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

    // ── agentpod ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn update_agentpod_settings() {
        let s = MockServer::start().await;
        Mock::given(method("PUT")).and(path("/api/v1/users/me/agentpod/settings"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let data = agentsmesh_types::AgentPodSettings {
            default_runner_id: Some(1),
            default_agent_slug: None,
            preferences: None,
        };
        let _ = c.update_agentpod_settings(&data).await.unwrap();
    }

    #[tokio::test]
    async fn create_agentpod_provider() {
        let s = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/users/me/agentpod/providers"))
            .respond_with(ok(json!({"id":1,"name":"openai","provider_type":"openai"})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let data = agentsmesh_types::CreateAIProviderRequest {
            name: "openai".into(),
            provider_type: "openai".into(),
            base_url: None,
            api_key: Some("sk-test".into()),
        };
        let _ = c.create_agentpod_provider(&data).await.unwrap();
    }

    #[tokio::test]
    async fn update_agentpod_provider() {
        let s = MockServer::start().await;
        Mock::given(method("PUT")).and(path("/api/v1/users/me/agentpod/providers/5"))
            .respond_with(ok(json!({"id":5,"name":"updated","provider_type":"openai"})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let data = agentsmesh_types::UpdateAIProviderRequest {
            name: Some("updated".into()),
            base_url: None,
            api_key: None,
        };
        let _ = c.update_agentpod_provider(5, &data).await.unwrap();
    }

    #[tokio::test]
    async fn delete_agentpod_provider() {
        let s = MockServer::start().await;
        Mock::given(method("DELETE")).and(path("/api/v1/users/me/agentpod/providers/5"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let _ = c.delete_agentpod_provider(5).await.unwrap();
    }

    #[tokio::test]
    async fn set_default_agentpod_provider() {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/users/me/agentpod/providers/3/default"))
            .respond_with(ok(json!({})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::no_org());
        let _ = c.set_default_agentpod_provider(3).await.unwrap();
    }

    // ── autopilot ───────────────────────────────────────────────────────
    // REST surface dropped; covered by autopilot_connect.rs.

    // ── billing ─────────────────────────────────────────────────────────
    // Tests for REST endpoints owned by Connect-RPC removed
    // (subscription / plans / invoices / seats / overview). Connect handler
    // tests in backend/internal/api/connect/billing cover the same surface.
    // Only the remaining REST gaps (usage + quota) retain wiremock coverage.

    #[tokio::test]
    async fn get_billing_usage() {
        let s = MockServer::start().await;
        Mock::given(method("GET")).and(path("/api/v1/orgs/acme/billing/usage"))
            .and(query_param("type", "compute"))
            .respond_with(ok(json!({"usage":{"used":50,"total":100},"type":"compute"})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.get_billing_usage(Some("compute")).await.unwrap();
    }

    #[tokio::test]
    async fn check_billing_quota() {
        let s = MockServer::start().await;
        Mock::given(method("GET")).and(path("/api/v1/orgs/acme/billing/quota/check"))
            .and(query_param("resource", "pods"))
            .and(query_param("amount", "5"))
            .respond_with(ok(json!({"allowed":true})))
            .expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MockTokenStore::with_org("acme"));
        let _ = c.check_billing_quota("pods", Some(5)).await.unwrap();
    }
}
