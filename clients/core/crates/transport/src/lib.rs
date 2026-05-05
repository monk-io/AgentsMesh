mod error;
mod message;
pub mod runtime;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use error::TransportError;
pub use message::WsMessage;
pub use runtime::{BoxFuture, PlatformRuntime, Runtime, TaskHandle};

#[cfg(not(target_arch = "wasm32"))]
pub use native::{WebSocketConnection, WsReceiver, WsSender};
#[cfg(target_arch = "wasm32")]
pub use wasm::{WebSocketConnection, WsReceiver, WsSender};

#[cfg(test)]
mod error_tests;
#[cfg(test)]
mod message_tests;
#[cfg(test)]
mod native_tests;
