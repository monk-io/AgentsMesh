pub(crate) mod connection_loop;
pub mod event_types;
pub mod reconnect;
pub mod stream_source;
pub mod subscription_manager;
pub mod types;

#[cfg(test)]
mod tests;

pub use event_types::EventType;
pub use reconnect::ReconnectPolicy;
pub use stream_source::{ApiClientStreamSource, EventStreamSource};
pub use subscription_manager::EventSubscriptionManager;
pub use types::{
    ConnectionState, EventCategory, EventHandler, EventSubscriptionManagerOptions,
    RealtimeEvent, StateListener, SubscriptionId,
};
