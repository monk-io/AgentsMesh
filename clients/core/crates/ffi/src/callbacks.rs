#[uniffi::export(callback_interface)]
pub trait StorageCallback: Send + Sync {
    fn get(&self, key: String) -> Option<String>;
    fn set(&self, key: String, value: String);
    fn remove(&self, key: String);
}

#[uniffi::export(callback_interface)]
pub trait OutputCallback: Send + Sync {
    fn on_output(&self, pod_key: String, data: Vec<u8>);
}

#[uniffi::export(callback_interface)]
pub trait StatusCallback: Send + Sync {
    fn on_status_change(&self, pod_key: String, status: String, runner_disconnected: bool);
}

/// Relay ACP control-plane message (AcpEvent / AcpSnapshot). `msg_type` is the
/// wire MsgType byte; `payload_json` is the raw JSON the runner emitted.
#[uniffi::export(callback_interface)]
pub trait AcpCallback: Send + Sync {
    fn on_acp(&self, pod_key: String, msg_type: u8, payload_json: String);
}

/// Fired once when a pod connection is fully torn down so the Swift relay
/// adapter can drop its register-once guard and re-wire on the next subscribe.
#[uniffi::export(callback_interface)]
pub trait PodDisconnectedCallback: Send + Sync {
    fn on_pod_disconnected(&self, pod_key: String);
}


#[uniffi::export(callback_interface)]
pub trait EventCallback: Send + Sync {
    fn on_event(&self, event_json: String);
}

/// Fires once per dispatched realtime event AFTER `AppState.dispatch`
/// has applied the change. Swift/Kotlin consumers maintain an
/// `@Observable` tick counter that triggers UI re-derivation via
/// selector reads — no event JSON parsing on the platform side.
///
/// Contract: `on_tick` may run on any thread. Swift implementations
/// MUST hop to `@MainActor` (or equivalent) before mutating
/// SwiftUI-observable state to avoid threading traps.
#[uniffi::export(callback_interface)]
pub trait TickCallback: Send + Sync {
    fn on_tick(&self, tick: u64);
}

/// Fires whenever the realtime stream's connection state changes
/// (connecting / connected / reconnecting / disconnected). iOS keeps an
/// `@Observable` store and shows a reconnect banner when the state is not
/// "connected" past a debounce. Runs on a tokio worker — hop to `@MainActor`
/// before mutating SwiftUI state.
#[uniffi::export(callback_interface)]
pub trait ConnectionStateCallback: Send + Sync {
    fn on_connection_state_change(&self, state: String);
}
