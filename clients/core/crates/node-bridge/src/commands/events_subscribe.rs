use std::sync::Arc;

use agentsmesh_events::{EventHandler, RealtimeEvent, StateListener, SubscriptionId};
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;

use crate::AppState;

#[napi]
impl AppState {
    #[napi]
    pub async fn events_connect(&self) -> napi::Result<()> {
        let mut events = self.events.lock().await;
        events.connect().await;
        Ok(())
    }

    #[napi]
    pub async fn events_disconnect(&self) -> napi::Result<()> {
        let mut events = self.events.lock().await;
        events.disconnect().await;
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
        let events = self.events.lock().await;
        let id = events.subscribe_all(handler).await;
        Ok(id.as_u64() as f64)
    }

    #[napi]
    pub async fn events_unsubscribe(&self, id: f64) -> napi::Result<()> {
        let events = self.events.lock().await;
        events
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
        let events = self.events.lock().await;
        let id = events.on_connection_state_change(listener).await;
        Ok(id.as_u64() as f64)
    }

    #[napi]
    pub async fn events_get_connection_state(&self) -> napi::Result<String> {
        let events = self.events.lock().await;
        Ok(events.get_connection_state().await.to_string())
    }
}
