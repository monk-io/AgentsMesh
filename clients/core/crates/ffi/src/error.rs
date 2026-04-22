use agentsmesh_api_client::ApiError;
use agentsmesh_auth::AuthError;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum CoreError {
    #[error("auth error: {message}")]
    Auth { message: String },

    #[error("api error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("not connected: {pod_key}")]
    NotConnected { pod_key: String },

    #[error("internal: {message}")]
    Internal { message: String },
}

impl From<AuthError> for CoreError {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::NotAuthenticated => Self::Auth {
                message: "not authenticated".into(),
            },
            AuthError::Server {
                status, message, ..
            } => Self::Api { status, message },
            other => Self::Auth {
                message: other.to_string(),
            },
        }
    }
}

impl From<ApiError> for CoreError {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::AuthExpired => Self::Auth {
                message: "auth expired".into(),
            },
            ApiError::Http {
                status,
                server_message,
                status_text,
                ..
            } => Self::Api {
                status,
                message: server_message.unwrap_or(status_text),
            },
            other => Self::Internal {
                message: other.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal {
            message: e.to_string(),
        }
    }
}
