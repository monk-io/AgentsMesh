#[cfg(test)]
mod api_ticket_tests {
    use std::sync::{Arc, Mutex};

    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{ApiClient, AuthTokenStore};

    struct Tok(Mutex<Option<String>>);
    impl Tok {
        fn org(s: &str) -> Arc<Self> { Arc::new(Self(Mutex::new(Some(s.into())))) }
    }
    impl AuthTokenStore for Tok {
        fn get_token(&self) -> Option<String> { Some("tok".into()) }
        fn get_refresh_token(&self) -> Option<String> { None }
        fn set_tokens(&self, _t: String, _r: String, _e: Option<i64>) {}
        fn clear_tokens(&self) {}
        fn get_current_org_slug(&self) -> Option<String> { self.0.lock().unwrap().clone() }
    }

    fn ok(b: serde_json::Value) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(b)
    }

    // ticket REST mocks removed: REST surface eliminated; Connect handler
    // tests in backend/internal/api/connect/ticket cover ticket CRUD / board /
    // labels / assignees. get_ticket_pods stays on REST (MeshService owns it).

    #[tokio::test]
    async fn get_ticket_pods() {
        let s = MockServer::start().await;
        Mock::given(method("GET")).and(path("/api/v1/orgs/acme/tickets/TK-1/pods"))
            .and(query_param("active", "true"))
            .respond_with(ok(json!({"pods":[]}))).expect(1).mount(&s).await;
        let c = ApiClient::new(s.uri(), Tok::org("acme"));
        let _ = c.get_ticket_pods("TK-1", Some(true)).await.unwrap();
    }
}
