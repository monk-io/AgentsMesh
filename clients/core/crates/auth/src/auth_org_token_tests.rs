#[cfg(test)]
mod auth_org_token_tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use agentsmesh_api_client::AuthTokenStore;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::manager::AuthManager;
    use crate::state::AuthState;
    use crate::storage::PersistentStorage;

    struct InMemoryStorage {
        data: Mutex<HashMap<String, String>>,
    }

    impl InMemoryStorage {
        fn new() -> Arc<Self> {
            Arc::new(Self { data: Mutex::new(HashMap::new()) })
        }

        fn with_data(key: &str, value: &str) -> Arc<Self> {
            let mut map = HashMap::new();
            map.insert(key.into(), value.into());
            Arc::new(Self { data: Mutex::new(map) })
        }
    }

    impl PersistentStorage for InMemoryStorage {
        fn get(&self, key: &str) -> Option<String> {
            self.data.lock().unwrap().get(key).cloned()
        }
        fn set(&self, key: &str, value: &str) {
            self.data.lock().unwrap().insert(key.into(), value.into());
        }
        fn remove(&self, key: &str) {
            self.data.lock().unwrap().remove(key);
        }
        fn clear(&self) {
            self.data.lock().unwrap().clear();
        }
    }

    fn session_json() -> serde_json::Value {
        serde_json::json!({
            "token": "access-tok",
            "refresh_token": "refresh-tok",
            "user": {
                "id": 1, "email": "dev@test.com",
                "username": "dev", "name": "Dev User", "avatar_url": null
            },
            "expires_in": 3600
        })
    }

    #[tokio::test]
    async fn refresh_token_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("POST")).and(path("/api/v1/auth/refresh"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "token": "new-access", "refresh_token": "new-refresh", "expires_in": 7200
            })))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        let tokens = manager.refresh_token().await.unwrap();
        assert_eq!(tokens.token, "new-access");
        assert_eq!(tokens.refresh_token, "new-refresh");
    }

    #[tokio::test]
    async fn refresh_without_auth() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        let err = manager.refresh_token().await.unwrap_err();
        assert!(matches!(err, crate::AuthError::NotAuthenticated));
    }

    #[tokio::test]
    async fn fetch_organizations_selects_first() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("GET")).and(path("/api/v1/users/me/organizations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "organizations": [
                    {"id": 1, "name": "Org A", "slug": "org-a"},
                    {"id": 2, "name": "Org B", "slug": "org-b"}
                ]
            })))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        let orgs = manager.fetch_organizations().await.unwrap();
        assert_eq!(orgs.len(), 2);

        let current = manager.get_current_org().unwrap();
        assert_eq!(current.slug, "org-a");
        assert_eq!(manager.get_organizations().len(), 2);
    }

    #[tokio::test]
    async fn switch_org_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("GET")).and(path("/api/v1/users/me/organizations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "organizations": [
                    {"id": 1, "name": "Org A", "slug": "org-a"},
                    {"id": 2, "name": "Org B", "slug": "org-b"}
                ]
            })))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        manager.fetch_organizations().await.unwrap();
        manager.switch_org("org-b").unwrap();
        assert_eq!(manager.get_current_org().unwrap().slug, "org-b");
    }

    #[tokio::test]
    async fn switch_org_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("GET")).and(path("/api/v1/users/me/organizations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "organizations": [
                    {"id": 1, "name": "Org A", "slug": "org-a"}
                ]
            })))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        manager.fetch_organizations().await.unwrap();
        let err = manager.switch_org("nonexistent");
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn get_current_org_slug_via_trait() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("GET")).and(path("/api/v1/users/me/organizations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "organizations": [
                    {"id": 1, "name": "Org A", "slug": "org-a"}
                ]
            })))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        assert!(manager.get_current_org_slug().is_none());
        manager.login("dev@test.com", "pass").await.unwrap();
        manager.fetch_organizations().await.unwrap();
        assert_eq!(manager.get_current_org_slug(), Some("org-a".into()));
    }

    #[tokio::test]
    async fn set_current_org_directly() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        let org = agentsmesh_types::Organization {
            id: 99, name: "Direct Org".into(), slug: "direct-org".into(),
            role: None, logo_url: None, subscription_plan: None, subscription_status: None,
        };
        manager.set_current_org(org);
        let current = manager.get_current_org().unwrap();
        assert_eq!(current.slug, "direct-org");
    }

    #[tokio::test]
    async fn fetch_organizations_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("GET")).and(path("/api/v1/users/me/organizations"))
            .respond_with(ResponseTemplate::new(500)
                .set_body_json(serde_json::json!({"message": "internal"})))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        manager.login("dev@test.com", "pass").await.unwrap();
        let err = manager.fetch_organizations().await.unwrap_err();
        assert!(matches!(err, crate::AuthError::Server { status: 500, .. }));
    }

    #[tokio::test]
    async fn fetch_organizations_not_authenticated() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        let err = manager.fetch_organizations().await.unwrap_err();
        assert!(matches!(err, crate::AuthError::NotAuthenticated));
    }

    #[tokio::test]
    async fn auth_token_store_trait() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        assert!(manager.get_token().is_none());
        manager.login("dev@test.com", "pass").await.unwrap();
        assert_eq!(manager.get_token(), Some("access-tok".into()));
        assert_eq!(manager.get_refresh_token(), Some("refresh-tok".into()));

        manager.set_tokens("new-t".into(), "new-r".into());
        assert_eq!(manager.get_token(), Some("new-t".into()));
        assert_eq!(manager.get_refresh_token(), Some("new-r".into()));

        manager.clear_tokens();
        assert!(manager.get_token().is_none());
        assert!(manager.get_refresh_token().is_none());
    }

    #[test]
    fn bearer_header_with_token() {
        let state = AuthState {
            token: Some("my-tok".into()), refresh_token: None,
            user: None, current_org: None, organizations: vec![],
        };
        let json = serde_json::to_string(&state).unwrap();
        let storage = InMemoryStorage::with_data("agentsmesh-auth", &json);
        let manager = AuthManager::new("http://localhost".into(), storage);
        manager.restore_session().unwrap();
        let header = manager.bearer_header().unwrap();
        assert_eq!(header, "Bearer my-tok");
    }

    #[test]
    fn bearer_header_not_authenticated() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        let err = manager.bearer_header();
        assert!(err.is_err());
    }

    #[test]
    fn is_authenticated_states() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        assert!(!manager.is_authenticated());
        assert!(manager.current_user().is_none());
    }
}
