use std::sync::Arc;

use agentsmesh_protocol::MsgType;
use agentsmesh_relay::{AcpCallback, DisconnectCallback, OutputCallback, RelayStatusInfo, StatusCallback};
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;

use crate::AppState;

// Terminal data-plane relay surface over the shared `RelayConnectionPool` (the
// SSOT). The pool runs natively in the main process; `main/relay.ts` provides
// the `on_output`/`on_status`/`on_acp` ThreadsafeFunctions that fan bytes out to
// the renderer via `webContents.send`, and input/resize/acp come back as the
// `relay_*` commands below. Mirrors the WasmRelayManager surface (web) so the
// shared renderer relay adapter is platform-symmetric.
fn err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(e.to_string())
}

#[napi]
impl AppState {
    #[napi]
    pub async fn relay_subscribe(
        &self,
        pod_key: String,
        subscription_id: String,
        relay_url: String,
        token: String,
        on_output: ThreadsafeFunction<Vec<u8>>,
    ) -> napi::Result<()> {
        let cb = Arc::new(on_output);
        let output_cb: OutputCallback = Arc::new(move |data: Vec<u8>| {
            cb.call(Ok(data), ThreadsafeFunctionCallMode::NonBlocking);
        });
        self.relay
            .subscribe(&pod_key, &subscription_id, &relay_url, &token, output_cb)
            .await;
        Ok(())
    }

    #[napi]
    pub async fn relay_unsubscribe(&self, pod_key: String, subscription_id: String) {
        self.relay.unsubscribe(&pod_key, &subscription_id).await;
    }

    #[napi]
    pub async fn relay_send(&self, pod_key: String, data: String) {
        self.relay.send(&pod_key, &data).await;
    }

    #[napi]
    pub async fn relay_send_resize(&self, pod_key: String, cols: u16, rows: u16) {
        self.relay.send_resize(&pod_key, cols, rows).await;
    }

    #[napi]
    pub async fn relay_force_resize(&self, pod_key: String, cols: u16, rows: u16) {
        self.relay.force_resize(&pod_key, cols, rows).await;
    }

    #[napi]
    pub async fn relay_send_acp_command(&self, pod_key: String, command: String) -> napi::Result<()> {
        let val: serde_json::Value = serde_json::from_str(&command).map_err(err)?;
        self.relay.send_acp_command(&pod_key, &val).await.map_err(err)
    }

    #[napi]
    pub async fn relay_disconnect(&self, pod_key: String) {
        self.relay.disconnect(&pod_key).await;
    }

    #[napi]
    pub async fn relay_disconnect_all(&self) {
        self.relay.disconnect_all().await;
    }

    #[napi]
    pub async fn relay_get_status(&self, pod_key: String) -> String {
        self.relay.get_status(&pod_key).await.to_string()
    }

    #[napi]
    pub async fn relay_is_runner_disconnected(&self, pod_key: String) -> bool {
        self.relay.is_runner_disconnected(&pod_key).await
    }

    /// `[cols, rows]` or empty when the pod size is unknown.
    #[napi]
    pub async fn relay_get_pod_size(&self, pod_key: String) -> Vec<u16> {
        self.relay
            .get_pod_size(&pod_key)
            .await
            .map(|(c, r)| vec![c, r])
            .unwrap_or_default()
    }

    /// Status callback delivers `{"status","runnerDisconnected"}` JSON; main
    /// forwards to the renderer as a `relay:status` IPC event.
    #[napi]
    pub async fn relay_on_status_change(
        &self,
        pod_key: String,
        on_status: ThreadsafeFunction<String>,
    ) -> napi::Result<()> {
        let cb = Arc::new(on_status);
        let listener: StatusCallback = Arc::new(move |info: RelayStatusInfo| {
            let json = serde_json::json!({
                "status": info.status.to_string(),
                "runnerDisconnected": info.runner_disconnected,
            })
            .to_string();
            cb.call(Ok(json), ThreadsafeFunctionCallMode::NonBlocking);
        });
        self.relay.on_status_change(&pod_key, listener).await;
        Ok(())
    }

    /// ACP callback delivers `{"msgType","payload"}` JSON; main forwards as a
    /// `relay:acp` IPC event for the renderer's ACP dispatcher.
    #[napi]
    pub async fn relay_on_acp_message(
        &self,
        pod_key: String,
        on_acp: ThreadsafeFunction<String>,
    ) -> napi::Result<()> {
        let cb = Arc::new(on_acp);
        let listener: AcpCallback = Arc::new(move |msg_type: MsgType, payload: serde_json::Value| {
            let json = serde_json::json!({
                "msgType": msg_type as u8,
                "payload": payload,
            })
            .to_string();
            cb.call(Ok(json), ThreadsafeFunctionCallMode::NonBlocking);
        });
        self.relay.on_acp_message(&pod_key, listener).await;
        Ok(())
    }

    /// Pod-disconnected sink — `(podKey: string) => void`; main forwards as a
    /// `relay:pod-disconnected` IPC event so the renderer adapter resets its
    /// register-once guard and re-wires status/ACP on the next subscribe.
    #[napi]
    pub async fn relay_on_pod_disconnected(
        &self,
        on_disconnect: ThreadsafeFunction<String>,
    ) -> napi::Result<()> {
        let cb = Arc::new(on_disconnect);
        let listener: DisconnectCallback = Arc::new(move |pod_key: String| {
            cb.call(Ok(pod_key), ThreadsafeFunctionCallMode::NonBlocking);
        });
        self.relay.set_on_pod_disconnected(listener);
        Ok(())
    }
}
