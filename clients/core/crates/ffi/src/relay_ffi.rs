use std::sync::Arc;

use agentsmesh_protocol::MsgType;
use agentsmesh_relay::{RelayConnectionPool, RelayStatusInfo};
use agentsmesh_transport::runtime::PlatformRuntime;

use crate::callbacks::{AcpCallback, OutputCallback, StatusCallback};
use crate::error::CoreError;

/// Terminal data-plane SSOT for iOS — a uniffi wrapper over the shared Rust
/// `RelayConnectionPool` (the same pool web drives via WasmRelayManager). The
/// pool owns reconnect/backoff, input dedup, resize debounce, snapshot replay,
/// and codec; Swift keeps only SwiftTerm rendering + the TerminalReducer state
/// machine. Output/status/ACP flow back through uniffi callback interfaces.
///
/// The pool's connection tasks spawn on the tokio runtime uniffi provides for
/// the async-exported methods (async_runtime = "tokio"); the constructor stays
/// spawn-free. The auto-unsubscribe channel (ConnectionHandle drop) is unused
/// on iOS — Swift unsubscribes explicitly — so the receiver is dropped.
#[derive(uniffi::Object)]
pub struct RelayManager {
    pool: RelayConnectionPool<PlatformRuntime>,
}

#[uniffi::export(async_runtime = "tokio")]
impl RelayManager {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        let (pool, _rx) = RelayConnectionPool::with_runtime(PlatformRuntime);
        Arc::new(Self { pool })
    }

    pub async fn subscribe(
        &self,
        pod_key: String,
        subscription_id: String,
        relay_url: String,
        token: String,
        callback: Box<dyn OutputCallback>,
    ) {
        let cb: Arc<Box<dyn OutputCallback>> = Arc::new(callback);
        let pk = pod_key.clone();
        let output: agentsmesh_relay::OutputCallback = Arc::new(move |data: Vec<u8>| {
            cb.on_output(pk.clone(), data);
        });
        self.pool
            .subscribe(&pod_key, &subscription_id, &relay_url, &token, output)
            .await;
    }

    pub async fn unsubscribe(&self, pod_key: String, subscription_id: String) {
        self.pool.unsubscribe(&pod_key, &subscription_id).await;
    }

    pub async fn send(&self, pod_key: String, data: String) {
        self.pool.send(&pod_key, &data).await;
    }

    pub async fn send_resize(&self, pod_key: String, cols: u16, rows: u16) {
        self.pool.send_resize(&pod_key, cols, rows).await;
    }

    pub async fn force_resize(&self, pod_key: String, cols: u16, rows: u16) {
        self.pool.force_resize(&pod_key, cols, rows).await;
    }

    pub async fn send_acp_command(&self, pod_key: String, command_json: String) -> Result<(), CoreError> {
        let val: serde_json::Value = serde_json::from_str(&command_json)?;
        self.pool
            .send_acp_command(&pod_key, &val)
            .await
            .map_err(|e| CoreError::Unknown { message: e.to_string() })
    }

    pub async fn on_status_change(&self, pod_key: String, callback: Box<dyn StatusCallback>) {
        let cb: Arc<Box<dyn StatusCallback>> = Arc::new(callback);
        let pk = pod_key.clone();
        let listener: agentsmesh_relay::StatusCallback = Arc::new(move |info: RelayStatusInfo| {
            cb.on_status_change(pk.clone(), info.status.to_string(), info.runner_disconnected);
        });
        self.pool.on_status_change(&pod_key, listener).await;
    }

    pub async fn on_acp_message(&self, pod_key: String, callback: Box<dyn AcpCallback>) {
        let cb: Arc<Box<dyn AcpCallback>> = Arc::new(callback);
        let pk = pod_key.clone();
        let listener: agentsmesh_relay::AcpCallback = Arc::new(move |mt: MsgType, payload: serde_json::Value| {
            cb.on_acp(pk.clone(), mt as u8, payload.to_string());
        });
        self.pool.on_acp_message(&pod_key, listener).await;
    }

    pub async fn get_status(&self, pod_key: String) -> String {
        self.pool.get_status(&pod_key).await.to_string()
    }

    pub async fn is_runner_disconnected(&self, pod_key: String) -> bool {
        self.pool.is_runner_disconnected(&pod_key).await
    }

    pub async fn get_pod_size(&self, pod_key: String) -> Option<Vec<u16>> {
        self.pool.get_pod_size(&pod_key).await.map(|(c, r)| vec![c, r])
    }

    pub async fn disconnect(&self, pod_key: String) {
        self.pool.disconnect(&pod_key).await;
    }

    pub async fn disconnect_all(&self) {
        self.pool.disconnect_all().await;
    }
}
