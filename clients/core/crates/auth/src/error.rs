use agentsmesh_types::ServiceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("not authenticated")]
    NotAuthenticated,

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("server error: {status} - {message}")]
    Server {
        status: u16,
        message: String,
        code: Option<String>,
    },

    #[error("storage error: {0}")]
    Storage(String),
}

impl From<&AuthError> for ServiceError {
    fn from(e: &AuthError) -> Self {
        match e {
            AuthError::NotAuthenticated => ServiceError::AuthExpired,
            AuthError::Http(e) => ServiceError::Network {
                message: e.to_string(),
            },
            AuthError::InvalidResponse(msg) => ServiceError::Http {
                status: 0,
                code: None,
                message: msg.clone(),
            },
            AuthError::Server {
                status,
                message,
                code,
            } => {
                if *status == 401 {
                    return ServiceError::AuthExpired;
                }
                if *status == 404 {
                    return ServiceError::ResourceNotFound {
                        resource: code.clone().unwrap_or_else(|| "resource".into()),
                        id: None,
                    };
                }
                ServiceError::Http {
                    status: *status,
                    code: code.clone(),
                    message: message.clone(),
                }
            }
            AuthError::Storage(msg) => ServiceError::Unknown {
                message: msg.clone(),
            },
        }
    }
}

impl From<AuthError> for ServiceError {
    fn from(e: AuthError) -> Self {
        ServiceError::from(&e)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct ServerErrorBody {
    pub message: Option<String>,
    pub code: Option<String>,
}

pub(crate) async fn parse_error_response(resp: reqwest::Response) -> AuthError {
    let status = resp.status().as_u16();
    match resp.json::<ServerErrorBody>().await {
        Ok(body) => AuthError::Server {
            status,
            message: body.message.unwrap_or_else(|| "unknown error".into()),
            code: body.code,
        },
        Err(_) => AuthError::Server {
            status,
            message: "failed to parse error response".into(),
            code: None,
        },
    }
}
