uniffi::setup_scaffolding!();

mod auth_ffi;
mod callbacks;
mod core;
mod dto;
mod error;
mod relay_ffi;
mod services;
mod storage_bridge;

pub use callbacks::{AcpCallback, EventCallback, OutputCallback, StatusCallback, StorageCallback};
pub use self::core::AgentsMeshCore;
pub use error::CoreError;
pub use relay_ffi::RelayManager;

#[cfg(test)]
mod tests;
