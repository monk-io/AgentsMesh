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
