use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use agentsmesh_api_client::ApiError;
use agentsmesh_auth::{AuthError, PersistentStorage};

use crate::callbacks::StorageCallback;
use crate::core::AgentsMeshCore;
use crate::error::CoreError;
use crate::storage_bridge::StorageBridge;

struct MockStorage {
    data: Mutex<HashMap<String, String>>,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    fn with_entry(key: &str, value: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(key.to_string(), value.to_string());
        Self {
            data: Mutex::new(map),
        }
    }
}

impl StorageCallback for MockStorage {
    fn get(&self, key: String) -> Option<String> {
        self.data.lock().unwrap().get(&key).cloned()
    }

    fn set(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }

    fn remove(&self, key: String) {
        self.data.lock().unwrap().remove(&key);
    }

    fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
}

fn make_core() -> AgentsMeshCore {
    AgentsMeshCore::new("https://example.com".into(), Box::new(MockStorage::new()))
}

fn make_core_with_storage(storage: MockStorage) -> AgentsMeshCore {
    AgentsMeshCore::new("https://example.com".into(), Box::new(storage))
}

// ── StorageBridge ──

#[test]
fn bridge_get_set_remove() {
    let mock = Arc::new(MockStorage::new());
    let bridge = StorageBridge::new(mock.clone());

    assert_eq!(bridge.get("k"), None);

    bridge.set("k", "v");
    assert_eq!(bridge.get("k"), Some("v".into()));

    bridge.remove("k");
    assert_eq!(bridge.get("k"), None);
}

#[test]
fn bridge_clear() {
    let mock = Arc::new(MockStorage::with_entry("a", "1"));
    let bridge = StorageBridge::new(mock.clone());

    assert!(bridge.get("a").is_some());
    bridge.clear();
    assert_eq!(bridge.get("a"), None);
}

// ── CoreError From<AuthError> ──

#[test]
fn from_auth_not_authenticated() {
    let err: CoreError = AuthError::NotAuthenticated.into();
    assert!(matches!(err, CoreError::AuthExpired));
}

#[test]
fn from_auth_server_error() {
    let err: CoreError = AuthError::Server {
        status: 422,
        message: "validation failed".into(),
        code: Some("INVALID".into()),
    }
    .into();
    match err {
        CoreError::Http {
            status,
            code,
            message,
        } => {
            assert_eq!(status, 422);
            assert_eq!(code.as_deref(), Some("INVALID"));
            assert_eq!(message, "validation failed");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

#[test]
fn from_auth_storage_error() {
    let err: CoreError = AuthError::Storage("corrupt".into()).into();
    match err {
        CoreError::Unknown { message } => assert!(message.contains("corrupt")),
        other => panic!("expected Unknown, got {other:?}"),
    }
}

// ── CoreError From<ApiError> ──

#[test]
fn from_api_auth_expired() {
    let err: CoreError = ApiError::AuthExpired.into();
    assert!(matches!(err, CoreError::AuthExpired));
}

#[test]
fn from_api_http_with_server_message() {
    let err: CoreError = ApiError::Http {
        status: 400,
        status_text: "Bad Request".into(),
        code: None,
        server_message: Some("field invalid".into()),
        data: None,
    }
    .into();
    match err {
        CoreError::Http {
            status, message, ..
        } => {
            assert_eq!(status, 400);
            assert_eq!(message, "field invalid");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

#[test]
fn from_api_http_falls_back_to_status_text() {
    let err: CoreError = ApiError::Http {
        status: 500,
        status_text: "Internal Server Error".into(),
        code: None,
        server_message: None,
        data: None,
    }
    .into();
    match err {
        CoreError::Http {
            status, message, ..
        } => {
            assert_eq!(status, 500);
            assert_eq!(message, "Internal Server Error");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

#[test]
fn from_api_json_error_becomes_invalid_json() {
    let json_err = serde_json::from_str::<i32>("not json").unwrap_err();
    let err: CoreError = ApiError::Json(json_err).into();
    assert!(matches!(err, CoreError::InvalidJson { .. }));
}

// ── CoreError From<serde_json::Error> ──

#[test]
fn from_serde_json_error() {
    let json_err = serde_json::from_str::<i32>("bad").unwrap_err();
    let err: CoreError = json_err.into();
    match err {
        CoreError::InvalidJson { message } => assert!(!message.is_empty()),
        other => panic!("expected InvalidJson, got {other:?}"),
    }
}

// ── CoreError Display ──

#[test]
fn display_auth_expired() {
    assert_eq!(CoreError::AuthExpired.to_string(), "auth expired");
}

#[test]
fn display_http_error() {
    let err = CoreError::Http {
        status: 404,
        code: None,
        message: "not found".into(),
    };
    assert_eq!(err.to_string(), "HTTP 404: not found");
}

#[test]
fn display_not_connected_error() {
    let err = CoreError::NotConnected {
        pod_key: "pod-123".into(),
    };
    assert_eq!(err.to_string(), "not connected: pod-123");
}

#[test]
fn display_unknown_error() {
    let err = CoreError::Unknown {
        message: "boom".into(),
    };
    assert_eq!(err.to_string(), "boom");
}

// ── AgentsMeshCore ──

#[test]
fn core_new_succeeds() {
    let core = make_core();
    assert!(!core.is_authenticated());
}

#[test]
fn core_initial_state_no_user() {
    let core = make_core();
    assert!(core.get_current_user_json().is_none());
    assert!(core.get_current_org_json().is_none());
}

#[test]
fn core_get_organizations_empty() {
    let core = make_core();
    let json = core.get_organizations_json().unwrap();
    assert_eq!(json, "[]");
}

#[test]
fn restore_session_empty_storage_returns_false() {
    let core = make_core();
    assert_eq!(core.restore_session().unwrap(), false);
}

#[test]
fn restore_session_with_valid_state() {
    let state_json = serde_json::json!({
        "token": "tok_abc",
        "refresh_token": "ref_xyz",
        "user": {
            "id": 1,
            "email": "test@example.com",
            "username": "tester",
            "name": "Test User"
        },
        "organizations": [],
        "current_org": null
    });

    let storage = MockStorage::with_entry("agentsmesh-auth", &state_json.to_string());
    let core = make_core_with_storage(storage);

    assert!(!core.is_authenticated());
    let restored = core.restore_session().unwrap();
    assert!(restored);
    assert!(core.is_authenticated());
    assert!(core.get_current_user_json().is_some());
}

#[test]
fn restore_session_with_no_token_returns_false() {
    let state_json = serde_json::json!({
        "token": null,
        "refresh_token": null,
        "user": null,
        "organizations": [],
        "current_org": null
    });

    let storage = MockStorage::with_entry("agentsmesh-auth", &state_json.to_string());
    let core = make_core_with_storage(storage);

    assert_eq!(core.restore_session().unwrap(), false);
    assert!(!core.is_authenticated());
}

#[test]
fn restore_session_with_corrupt_json_returns_error() {
    let storage = MockStorage::with_entry("agentsmesh-auth", "not valid json");
    let core = make_core_with_storage(storage);

    let result = core.restore_session();
    assert!(result.is_err());
}

// ── relay_ffi ──

#[test]
fn relay_placeholder_returns_expected() {
    let core = make_core();
    assert_eq!(core.relay_placeholder(), "relay integration pending");
}

// ── api_ffi: api_org_path ──

#[test]
fn api_org_path_without_org() {
    let core = make_core();
    let path = core.api_org_path("/pods".into());
    assert!(path.contains("/pods"));
}

#[test]
fn api_org_path_with_org() {
    let state_json = serde_json::json!({
        "token": "tok",
        "refresh_token": "ref",
        "user": { "id": 1, "email": "u@x.com", "username": "u", "name": "U" },
        "organizations": [{ "id": 1, "name": "Org", "slug": "test-org" }],
        "current_org": { "id": 1, "name": "Org", "slug": "test-org" }
    });
    let storage = MockStorage::with_entry("agentsmesh-auth", &state_json.to_string());
    let core = make_core_with_storage(storage);
    core.restore_session().unwrap();
    core.switch_org("test-org".into()).unwrap();
    let path = core.api_org_path("/pods".into());
    assert!(path.contains("test-org"));
}

// ── api_ffi: invalid JSON body triggers CoreError::InvalidJson ──

#[tokio::test]
async fn api_post_invalid_body_returns_error() {
    let core = make_core();
    let result = core.api_post("/endpoint".into(), "not json".into()).await;
    match result {
        Err(CoreError::InvalidJson { .. }) => {}
        other => panic!("expected InvalidJson error from bad JSON body, got {other:?}"),
    }
}

#[tokio::test]
async fn api_put_invalid_body_returns_error() {
    let core = make_core();
    let result = core.api_put("/endpoint".into(), "{bad".into()).await;
    match result {
        Err(CoreError::InvalidJson { .. }) => {}
        other => panic!("expected InvalidJson error from bad JSON body, got {other:?}"),
    }
}

#[tokio::test]
async fn api_patch_invalid_body_returns_error() {
    let core = make_core();
    let result = core.api_patch("/endpoint".into(), "???".into()).await;
    match result {
        Err(CoreError::InvalidJson { .. }) => {}
        other => panic!("expected InvalidJson error from bad JSON body, got {other:?}"),
    }
}

// ── auth_ffi: switch_org error path ──

#[test]
fn switch_org_nonexistent_returns_error() {
    let core = make_core();
    let result = core.switch_org("no-such-org".into());
    assert!(result.is_err());
}

// ── CoreError: remaining AuthError variants ──

#[test]
fn from_auth_invalid_response() {
    let err: CoreError = AuthError::InvalidResponse("bad body".into()).into();
    match err {
        CoreError::Http {
            status, message, ..
        } => {
            assert_eq!(status, 0);
            assert!(message.contains("bad body"));
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

#[test]
fn from_auth_server_with_code() {
    let err: CoreError = AuthError::Server {
        status: 403,
        message: "forbidden".into(),
        code: Some("FORBIDDEN".into()),
    }
    .into();
    match err {
        CoreError::Http {
            status,
            code,
            message,
        } => {
            assert_eq!(status, 403);
            assert_eq!(code.as_deref(), Some("FORBIDDEN"));
            assert_eq!(message, "forbidden");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

// ── CoreError: ApiError with code and data ──

#[test]
fn from_api_http_with_code_and_data() {
    let err: CoreError = ApiError::Http {
        status: 422,
        status_text: "Unprocessable".into(),
        code: Some("VALIDATION_ERROR".into()),
        server_message: Some("field invalid".into()),
        data: Some(serde_json::json!({"field": "email"})),
    }
    .into();
    match err {
        CoreError::Http {
            status,
            code,
            message,
        } => {
            assert_eq!(status, 422);
            assert_eq!(code.as_deref(), Some("VALIDATION_ERROR"));
            assert_eq!(message, "field invalid");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

// ── CoreError: 404 maps to NotFound ──

#[test]
fn from_api_404_becomes_not_found() {
    let err: CoreError = ApiError::Http {
        status: 404,
        status_text: "Not Found".into(),
        code: Some("Pod".into()),
        server_message: None,
        data: None,
    }
    .into();
    match err {
        CoreError::NotFound { resource, id } => {
            assert_eq!(resource, "Pod");
            assert!(id.is_none());
        }
        other => panic!("expected NotFound, got {other:?}"),
    }
}
