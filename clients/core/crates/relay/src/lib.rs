mod command;
mod connection;
pub mod dispatch;
mod driver;
pub mod error;
pub mod pool;
pub mod retry;
pub mod types;

pub use error::RelayError;
pub use pool::RelayConnectionPool;
pub use types::{
    AcpCallback, ConnectionHandle, DisconnectCallback, OutputCallback, RelayStatus, RelayStatusInfo,
    StatusCallback,
};

#[cfg(test)]
mod dispatch_tests;
#[cfg(test)]
mod integration_tests;
