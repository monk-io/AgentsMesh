use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP {status}: {status_text}")]
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
}
