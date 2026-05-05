#[cfg(test)]
mod auth_api_error_tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

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
    async fn verify_email_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/verify-email"))
            .respond_with(ResponseTemplate::new(200).set_body_json(session_json()))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let session = manager.verify_email("tok-abc").await.unwrap();
        assert_eq!(session.token, "access-tok");
        assert!(manager.is_authenticated());
    }

    #[tokio::test]
    async fn verify_email_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/verify-email"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({"message": "invalid token"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.verify_email("bad-tok").await.unwrap_err();
        assert!(matches!(err, crate::AuthError::Server { status: 400, .. }));
    }

    #[tokio::test]
    async fn forgot_password_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/forgot-password"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);
        manager.forgot_password("user@test.com").await.unwrap();
    }

    #[tokio::test]
    async fn forgot_password_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/forgot-password"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_json(serde_json::json!({"message": "rate limited"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.forgot_password("user@test.com").await.unwrap_err();
        assert!(matches!(err, crate::AuthError::Server { status: 429, .. }));
    }

    #[tokio::test]
    async fn reset_password_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/reset-password"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);
        manager.reset_password("reset-tok", "newpass123").await.unwrap();
    }

    #[tokio::test]
    async fn reset_password_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/reset-password"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({"message": "token expired"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.reset_password("bad-tok", "newpass").await.unwrap_err();
        assert!(matches!(err, crate::AuthError::Server { status: 400, .. }));
    }

    #[tokio::test]
    async fn login_server_error_non_json_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(ResponseTemplate::new(502).set_body_string("Bad Gateway"))
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.login("a@b.com", "p").await.unwrap_err();
        match err {
            crate::AuthError::Server { status, message, code } => {
                assert_eq!(status, 502);
                assert_eq!(message, "failed to parse error response");
                assert!(code.is_none());
            }
            _ => panic!("expected Server error"),
        }
    }

    #[tokio::test]
    async fn login_server_error_no_message_field() {
        let server = MockServer::start().await;
        Mock::given(method("POST")).and(path("/api/v1/auth/login"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({"code": "INTERNAL"})),
            )
            .mount(&server).await;

        let storage = InMemoryStorage::new();
        let manager = AuthManager::new(server.uri(), storage);

        let err = manager.login("a@b.com", "p").await.unwrap_err();
        match err {
            crate::AuthError::Server { status, message, code } => {
                assert_eq!(status, 500);
                assert_eq!(message, "unknown error");
                assert_eq!(code, Some("INTERNAL".into()));
            }
            _ => panic!("expected Server error"),
        }
    }

    #[test]
    fn auth_error_display() {
        let not_auth = crate::AuthError::NotAuthenticated;
        assert_eq!(not_auth.to_string(), "not authenticated");

        let invalid = crate::AuthError::InvalidResponse("bad json".into());
        assert_eq!(invalid.to_string(), "invalid response: bad json");

        let server = crate::AuthError::Server {
            status: 404, message: "not found".into(), code: Some("NOT_FOUND".into()),
        };
        assert_eq!(server.to_string(), "server error: 404 - not found");

        let storage = crate::AuthError::Storage("disk full".into());
        assert_eq!(storage.to_string(), "storage error: disk full");
    }

    #[test]
    fn auth_state_clear() {
        let mut state = AuthState {
            token: Some("t".into()),
            refresh_token: Some("r".into()),
            user: Some(agentsmesh_types::User {
                id: 1, email: "e".into(), username: "u".into(),
                name: None, avatar_url: None,
            }),
            current_org: None,
            organizations: vec![],
        };
        state.clear();
        assert!(state.token.is_none());
        assert!(state.user.is_none());
    }

    #[test]
    fn auth_state_apply_session() {
        let mut state = AuthState::default();
        let session = agentsmesh_types::AuthSession {
            token: "t".into(), refresh_token: "r".into(),
            user: agentsmesh_types::User {
                id: 1, email: "e".into(), username: "u".into(),
                name: None, avatar_url: None,
            },
            expires_in: None,
        };
        state.apply_session(&session);
        assert_eq!(state.token, Some("t".into()));
        assert_eq!(state.user.as_ref().unwrap().id, 1);
    }

    #[test]
    fn auth_state_apply_tokens() {
        let mut state = AuthState::default();
        let tokens = agentsmesh_types::AuthTokens {
            token: "new-t".into(), refresh_token: "new-r".into(), expires_in: Some(3600),
        };
        state.apply_tokens(&tokens);
        assert_eq!(state.token, Some("new-t".into()));
        assert_eq!(state.refresh_token, Some("new-r".into()));
    }

    #[test]
    fn in_memory_storage_operations() {
        let storage = InMemoryStorage::new();
        assert!(storage.get("key").is_none());

        storage.set("key", "value");
        assert_eq!(storage.get("key"), Some("value".into()));

        storage.remove("key");
        assert!(storage.get("key").is_none());

        storage.set("a", "1");
        storage.set("b", "2");
        storage.clear();
        assert!(storage.get("a").is_none());
        assert!(storage.get("b").is_none());
    }
}
