#[cfg(test)]
mod api_billing_extra_tests {
    use std::sync::{Arc, Mutex};
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use crate::{ApiClient, AuthTokenStore};
    use agentsmesh_types::*;

    struct MT { org: Mutex<Option<String>> }
    impl MT { fn org(s: &str) -> Arc<Self> { Arc::new(Self { org: Mutex::new(Some(s.into())) }) } }
    impl AuthTokenStore for MT {
        fn get_token(&self) -> Option<String> { Some("t".into()) }
        fn get_refresh_token(&self) -> Option<String> { None }
        fn set_tokens(&self, _: String, _: String, _: Option<i64>) {}
        fn clear_tokens(&self) {}
        fn get_current_org_slug(&self) -> Option<String> { self.org.lock().unwrap().clone() }
    }
    fn ok(b: serde_json::Value) -> ResponseTemplate { ResponseTemplate::new(200).set_body_json(b) }

    // REST tests for endpoints owned by Connect-RPC removed — coverage lives
    // in backend/internal/api/connect/billing handler tests. Only the REST
    // gaps (customer portal) retain wiremock coverage here.

    #[tokio::test]
    async fn get_customer_portal() {
        let s = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/orgs/a/billing/customer-portal"))
            .respond_with(ok(json!({"url":"x"}))).expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), MT::org("a"));
        let _ = c.get_customer_portal(&CustomerPortalRequest { return_url: Some("x".into()) }).await;
    }
}
