use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelayError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("not connected: {0}")]
    NotConnected(String),

    #[error("send error: {0}")]
    Send(String),

    #[error("protocol error: {0}")]
    Protocol(#[from] agentsmesh_protocol::ProtocolError),
}
