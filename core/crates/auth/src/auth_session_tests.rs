#[cfg(test)]
mod auth_session_tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use agentsmesh_api_client::AuthTokenStore;
    use agentsmesh_types::RegisterRequest;
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
    async fn login_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage.clone());

        let session = manager.login("dev@test.com", "pass").await.unwrap();
        assert_eq!(session.token, "access-tok");
        assert_eq!(session.user.email, "dev@test.com");
        assert!(manager.is_authenticated());
        assert!(storage.get("agentsmesh-auth").is_some());
    }

    #[tokio::test]
    async fn login_failure_401() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({"message": "invalid credentials"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.login("bad@test.com", "wrong").await.unwrap_err();
        match err {
            crate::AuthError::Server { status, message, .. } => {
                assert_eq!(status, 401);
                assert!(message.contains("invalid credentials"));
            }
            _ => panic!("expected Server error, got: {err:?}"),
        }
    }

    #[tokio::test]
    async fn register_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/register"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let req = RegisterRequest {
            name: "Dev".into(), email: "dev@test.com".into(),
            username: "dev".into(), password: "pass123".into(),
        };
        let session = manager.register(&req).await.unwrap();
        assert_eq!(session.user.username, "dev");
        assert!(manager.is_authenticated());
    }

    #[tokio::test]
    async fn register_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/register"))
            .respond_with(
                ResponseTemplate::new(409)
                    .set_body_json(serde_json::json!({"message": "email taken"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let req = RegisterRequest {
            name: "Dev".into(), email: "dup@test.com".into(),
            username: "dev".into(), password: "pass123".into(),
        };
        let err = manager.register(&req).await.unwrap_err();
        assert!(matches!(err, crate::AuthError::Server { status: 409, .. }));
    }

    #[tokio::test]
    async fn logout_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;
        Mock::given(method("POST")).and(path("/api/v1/auth/logout"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage.clone());

        manager.login("dev@test.com", "pass").await.unwrap();
        assert!(manager.is_authenticated());

        manager.logout().await.unwrap();
        assert!(!manager.is_authenticated());
        assert!(manager.current_user().is_none());
        assert!(storage.get("agentsmesh-auth").is_none());
    }

    #[tokio::test]
    async fn logout_without_auth() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        let err = manager.logout().await.unwrap_err();
        assert!(matches!(err, crate::AuthError::NotAuthenticated));
    }

    #[test]
    fn restore_session_with_data() {
        let state = AuthState {
            token: Some("saved-tok".into()),
            refresh_token: Some("saved-ref".into()),
            user: Some(agentsmesh_types::User {
                id: 1, email: "dev@test.com".into(),
                username: "dev".into(), name: None, avatar_url: None,
            }),
            current_org: None,
            organizations: vec![],
        };
        let json = serde_json::to_string(&state).unwrap();
        let storage = InMemoryStorage::with_data("agentsmesh-auth", &json);
        let manager = AuthManager::new("http://localhost".into(), storage);

        let restored = manager.restore_session().unwrap();
        assert!(restored);
        assert!(manager.is_authenticated());
        assert_eq!(manager.current_user().unwrap().email, "dev@test.com");
    }

    #[test]
    fn restore_session_empty() {
        let storage = InMemoryStorage::new();
        let manager = AuthManager::new("http://localhost".into(), storage);
        let restored = manager.restore_session().unwrap();
        assert!(!restored);
        assert!(!manager.is_authenticated());
    }

    #[test]
    fn restore_session_invalid_json() {
        let storage = InMemoryStorage::with_data("agentsmesh-auth", "not-valid-json{{{");
        let manager = AuthManager::new("http://localhost".into(), storage);
        let err = manager.restore_session();
        assert!(err.is_err());
    }

    #[test]
    fn restore_session_no_token() {
        let state = AuthState {
            token: None, refresh_token: None, user: None,
            current_org: None, organizations: vec![],
        };
        let json = serde_json::to_string(&state).unwrap();
        let storage = InMemoryStorage::with_data("agentsmesh-auth", &json);
        let manager = AuthManager::new("http://localhost".into(), storage);
        let restored = manager.restore_session().unwrap();
        assert!(!restored);
    }
}
