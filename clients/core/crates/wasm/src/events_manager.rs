use std::cell::RefCell;
use std::rc::Rc;

use agentsmesh_events::{
    EventSubscriptionManager, EventSubscriptionManagerOptions, EventType, SubscriptionId,
};
use agentsmesh_transport::runtime::PlatformRuntime;
use wasm_bindgen::prelude::*;

use crate::js_bridge::{make_event_handler, make_state_listener};

#[wasm_bindgen]
pub struct WasmEventsManager {
    inner: Rc<RefCell<EventSubscriptionManager<PlatformRuntime>>>,
}

#[wasm_bindgen]
impl WasmEventsManager {
    #[wasm_bindgen(constructor)]
    pub fn new(ws_url: String) -> Self {
        let manager = EventSubscriptionManager::with_runtime(
            PlatformRuntime,
            ws_url,
            EventSubscriptionManagerOptions::default(),
        );
        Self {
            inner: Rc::new(RefCell::new(manager)),
        }
    }

    pub fn new_with_options(
        ws_url: String,
        max_reconnect_attempts: u32,
        initial_reconnect_delay_ms: u32,
        max_reconnect_delay_ms: u32,
        ping_interval_ms: u32,
        pong_timeout_ms: u32,
    ) -> Self {
        let opts = EventSubscriptionManagerOptions {
            max_reconnect_attempts,
            initial_reconnect_delay_ms: initial_reconnect_delay_ms as u64,
            max_reconnect_delay_ms: max_reconnect_delay_ms as u64,
            ping_interval_ms: ping_interval_ms as u64,
            pong_timeout_ms: pong_timeout_ms as u64,
        };
        let manager = EventSubscriptionManager::with_runtime(PlatformRuntime, ws_url, opts);
        Self {
            inner: Rc::new(RefCell::new(manager)),
        }
    }

    pub async fn connect(&self) {
        self.inner.borrow_mut().connect().await;
    }

    pub async fn disconnect(&self) {
        self.inner.borrow_mut().disconnect().await;
    }

    pub async fn subscribe(
        &self,
        event_type: String,
        callback: js_sys::Function,
    ) -> Result<f64, String> {
        let et = parse_event_type(&event_type)?;
        let handler = make_event_handler(callback);
        let id = self.inner.borrow().subscribe(et, handler).await;
        Ok(sub_id_to_f64(id))
    }

    pub async fn subscribe_all(
        &self,
        callback: js_sys::Function,
    ) -> f64 {
        let handler = make_event_handler(callback);
        let id = self.inner.borrow().subscribe_all(handler).await;
        sub_id_to_f64(id)
    }

    pub async fn unsubscribe(&self, id: f64) {
        let sid = f64_to_sub_id(id);
        self.inner.borrow().unsubscribe(sid).await;
    }

    pub async fn on_connection_state_change(
        &self,
        callback: js_sys::Function,
    ) -> f64 {
        let listener = make_state_listener(callback);
        let id = self.inner.borrow().on_connection_state_change(listener).await;
        sub_id_to_f64(id)
    }

    pub async fn get_connection_state(&self) -> String {
        self.inner
            .borrow()
            .get_connection_state()
            .await
            .to_string()
    }
}

impl Default for WasmEventsManager {
    fn default() -> Self {
        Self::new(String::new())
    }
}

fn sub_id_to_f64(id: SubscriptionId) -> f64 {
    id.as_u64() as f64
}

fn f64_to_sub_id(id: f64) -> SubscriptionId {
    SubscriptionId::from_u64(id as u64)
}

fn parse_event_type(s: &str) -> Result<EventType, String> {
    let parsed: std::result::Result<EventType, _> =
        serde_json::from_value(serde_json::Value::String(s.to_string()));
    parsed.map_err(|e| format!("unknown event type '{s}': {e}"))
}
