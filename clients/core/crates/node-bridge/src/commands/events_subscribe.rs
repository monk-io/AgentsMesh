use std::sync::Arc;

use agentsmesh_events::{EventHandler, RealtimeEvent, StateListener, SubscriptionId};
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;

use crate::AppState;

#[napi]
impl AppState {
    #[napi]
    pub async fn events_connect(&self) -> napi::Result<()> {
        self.events.connect().await;
        Ok(())
    }

    #[napi]
    pub async fn events_disconnect(&self) -> napi::Result<()> {
        self.events.disconnect().await;
        Ok(())
    }

    /// Interrupt the reconnect backoff and retry now (network regained / app
    /// foregrounded). No-op when already connected or shut down.
    #[napi]
    pub async fn events_nudge(&self) -> napi::Result<()> {
        self.events.nudge().await;
        Ok(())
    }

    #[napi]
    pub async fn events_subscribe_all(
        &self,
        callback: ThreadsafeFunction<String>,
    ) -> napi::Result<f64> {
        let cb = Arc::new(callback);
        let handler: EventHandler = Arc::new({
            let cb = cb.clone();
            move |event: &RealtimeEvent| {
                if let Ok(json) = serde_json::to_string(event) {
                    cb.call(Ok(json), ThreadsafeFunctionCallMode::NonBlocking);
                }
            }
        });
        let id = self.events.subscribe_all(handler).await;
        Ok(id.as_u64() as f64)
    }

    #[napi]
    pub async fn events_unsubscribe(&self, id: f64) -> napi::Result<()> {
        self.events
            .unsubscribe(SubscriptionId::from_u64(id as u64))
            .await;
        Ok(())
    }

    #[napi]
    pub async fn events_on_connection_state_change(
        &self,
        callback: ThreadsafeFunction<String>,
    ) -> napi::Result<f64> {
        let cb = Arc::new(callback);
        let listener: StateListener = Arc::new(move |state| {
            cb.call(Ok(state.to_string()), ThreadsafeFunctionCallMode::NonBlocking);
        });
        let id = self.events.on_connection_state_change(listener).await;
        Ok(id.as_u64() as f64)
    }

    #[napi]
    pub async fn events_get_connection_state(&self) -> napi::Result<String> {
        Ok(self.events.get_connection_state().await.to_string())
    }

    /// Snapshot of the dispatch tick — increments after every event has
    /// been applied to AppState. Renderer reads via IPC and uses as the
    /// `useSyncExternalStore` snapshot for cache invalidation.
    #[napi]
    pub fn events_get_tick(&self) -> napi::Result<f64> {
        Ok(self.events.tick() as f64)
    }
}
