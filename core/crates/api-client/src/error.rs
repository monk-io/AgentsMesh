use thiserror::Error;

use agentsmesh_types::ServiceError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP {status}: {}", Self::describe_http(status_text, server_message.as_deref(), code.as_deref()))]
    Http {
        status: u16,
        status_text: String,
        code: Option<String>,
        server_message: Option<String>,
        data: Option<serde_json::Value>,
    },

    #[error("auth expired")]
    AuthExpired,

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl ApiError {
    pub fn has_code(&self, code: &str) -> bool {
        matches!(self, ApiError::Http { code: Some(c), .. } if c == code)
    }

    pub fn status(&self) -> Option<u16> {
        match self {
            ApiError::Http { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Structured, JSON wire-format representation for FFI boundaries.
    /// Use this instead of `.to_string()` in `map_err` so the front-end can
    /// discriminate on `kind` without regex-matching the human message.
    pub fn to_wire(&self) -> String {
        ServiceError::from(self).to_wire()
    }

    fn describe_http(status_text: &str, server_message: Option<&str>, code: Option<&str>) -> String {
        match (server_message, code) {
            (Some(msg), Some(c)) => format!("{msg} [{c}]"),
            (Some(msg), None) => msg.to_string(),
            (None, Some(c)) => format!("{status_text} [{c}]"),
            (None, None) => status_text.to_string(),
        }
    }
}

impl From<&ApiError> for ServiceError {
    fn from(e: &ApiError) -> Self {
        match e {
            ApiError::Http {
                status,
                status_text,
                code,
                server_message,
                ..
            } => {
                let message = server_message
                    .clone()
                    .unwrap_or_else(|| status_text.clone());
                if *status == 404 {
                    return ServiceError::ResourceNotFound {
                        resource: code
                            .clone()
                            .unwrap_or_else(|| "resource".to_string()),
                        id: None,
                    };
                }
                ServiceError::Http {
                    status: *status,
                    code: code.clone(),
                    message,
                }
            }
            ApiError::AuthExpired => ServiceError::AuthExpired,
            ApiError::Network(e) => ServiceError::Network {
                message: e.to_string(),
            },
            ApiError::Json(e) => ServiceError::InvalidJson {
                message: e.to_string(),
            },
        }
    }
}

impl From<ApiError> for ServiceError {
    fn from(e: ApiError) -> Self {
        ServiceError::from(&e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_404_maps_to_resource_not_found() {
        let err = ApiError::Http {
            status: 404,
            status_text: "Not Found".into(),
            code: Some("Pod".into()),
            server_message: Some("Pod not found".into()),
            data: None,
        };
        let svc: ServiceError = (&err).into();
        assert!(matches!(
            svc,
            ServiceError::ResourceNotFound { ref resource, .. } if resource == "Pod"
        ));
    }

    #[test]
    fn http_500_maps_to_http_variant() {
        let err = ApiError::Http {
            status: 500,
            status_text: "Internal Server Error".into(),
            code: Some("DB_DOWN".into()),
            server_message: Some("db unreachable".into()),
            data: None,
        };
        let svc: ServiceError = (&err).into();
        match svc {
            ServiceError::Http { status, code, message } => {
                assert_eq!(status, 500);
                assert_eq!(code.as_deref(), Some("DB_DOWN"));
                assert_eq!(message, "db unreachable");
            }
            _ => panic!("expected Http variant"),
        }
    }

    #[test]
    fn auth_expired_maps() {
        let svc: ServiceError = (&ApiError::AuthExpired).into();
        assert_eq!(svc, ServiceError::AuthExpired);
    }

    #[test]
    fn to_wire_emits_json() {
        let err = ApiError::AuthExpired;
        let wire = err.to_wire();
        assert_eq!(wire, r#"{"kind":"auth_expired"}"#);
    }
}
