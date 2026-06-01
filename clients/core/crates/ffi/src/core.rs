use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_auth::AuthManager;
use agentsmesh_events::types::TickListener;
use agentsmesh_events::{EventSubscriptionManager, EventSubscriptionManagerOptions};
use agentsmesh_services::BlockstoreService;
use agentsmesh_state::app_state::AppRuntime;
use agentsmesh_state::blockstore_state::BlockstoreState;
use agentsmesh_transport::runtime::PlatformRuntime;

use crate::callbacks::{StorageCallback, TickCallback};
use crate::dto::{BootstrapResultDto, OrganizationDto, UserDto};
use crate::error::CoreError;
use crate::storage_bridge::StorageBridge;

#[derive(uniffi::Object)]
pub struct AgentsMeshCore {
    pub(crate) auth: Arc<AuthManager>,
    pub(crate) api: Arc<ApiClient>,
    /// Local SSOT for the blockstore — mirrors what desktop/web have.
    /// Mutations + load_subtree go through this; sync flat-map readers
    /// (`blocks_json`, `refs_json`, ...) read from its in-process state
    /// without a backend round-trip, which is what the WebView-embedded
    /// React DocumentView expects.
    pub(crate) blockstore: Arc<BlockstoreService>,
    /// Realtime EventBus stream manager. The dispatch hook on this
    /// manager is wired into `runtime.state.dispatch` at construction
    /// time, so every event delivered via Connect-RPC server-stream
    /// flows directly into the shared `AppState`. Swift consumes via
    /// `set_tick_callback` (Phase 5).
    pub(crate) events: Arc<EventSubscriptionManager<PlatformRuntime>>,
    /// Singleton AppRuntime shared with services (Phase 2). Holds
    /// `Arc<RwLock<AppState>>` + the same events Arc above.
    pub(crate) runtime: Arc<AppRuntime>,
}

impl AgentsMeshCore {
    /// Resolve the current org slug for Connect-RPC requests that carry
    /// `org_slug` in the body (most proto.*.v1 services). Returns
    /// `CoreError::NotAuthenticated` rather than silently sending an
    /// empty string — the backend would 400, but the client error is
    /// clearer.
    pub(crate) fn org_slug(&self) -> Result<String, CoreError> {
        self.auth
            .get_current_org()
            .map(|o| o.slug)
            .ok_or_else(|| CoreError::Unknown {
                message: "no current org — Connect-RPC requires org_slug".into(),
            })
    }
}

#[uniffi::export]
impl AgentsMeshCore {
    #[uniffi::constructor]
    pub fn new(base_url: String, storage: Box<dyn StorageCallback>) -> Self {
        let bridge = Arc::new(StorageBridge::new(Arc::from(storage)));
        let auth = Arc::new(AuthManager::new(base_url.clone(), bridge));
        let api = Arc::new(ApiClient::new(base_url, auth.clone()));
        let blockstore = Arc::new(BlockstoreService::new(api.clone(), BlockstoreState::new()));
        let events = Arc::new(EventSubscriptionManager::with_runtime(
            PlatformRuntime,
            api.clone(),
            EventSubscriptionManagerOptions::default(),
        ));
        let runtime = AppRuntime::new(events.clone());
        Self { auth, api, blockstore, events, runtime }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth.is_authenticated()
    }

    /// Strongly-typed current-user accessor for Swift/Kotlin.
    pub fn get_current_user(&self) -> Option<UserDto> {
        self.auth.current_user().map(UserDto::from)
    }

    /// Strongly-typed current-org accessor for Swift/Kotlin.
    pub fn get_current_org(&self) -> Option<OrganizationDto> {
        self.auth.get_current_org().map(OrganizationDto::from)
    }

    /// Strongly-typed organization list accessor for Swift/Kotlin.
    pub fn get_organizations(&self) -> Vec<OrganizationDto> {
        self.auth
            .get_organizations()
            .into_iter()
            .map(OrganizationDto::from)
            .collect()
    }

    // ── Legacy JSON accessors ──
    // Retained for WASM/node-bridge parity until those frontends migrate off.

    pub fn get_current_user_json(&self) -> Option<String> {
        self.auth
            .current_user()
            .and_then(|u| serde_json::to_string(&u).ok())
    }

    pub fn get_current_org_json(&self) -> Option<String> {
        self.auth
            .get_current_org()
            .and_then(|o| serde_json::to_string(&o).ok())
    }

    pub fn get_organizations_json(&self) -> Result<String, CoreError> {
        let orgs = self.auth.get_organizations();
        Ok(serde_json::to_string(&orgs)?)
    }

    // ── Realtime event lifecycle (iOS) ──
    //
    // The events Connect-RPC stream is single-instance per `AgentsMeshCore`.
    // Swift calls `events_connect()` from `SceneDelegate.willEnterForeground`
    // and `events_disconnect()` from `didEnterBackground` to release the
    // socket while suspended. The dispatch hook (installed by `AppRuntime::new`)
    // mutates AppState before the tick callback fires; Swift reads selectors
    // (`pods_json()`, `channels_json()` etc on the runtime view) after each
    // tick to re-derive its `@Observable` stores.

    /// Register a tick listener that fires after every dispatched event.
    /// Replaces any prior listener. iOS uses this to invalidate SwiftUI
    /// observers; the callback runs on the dispatch thread — Swift
    /// implementations MUST hop to `@MainActor` before mutating UI state.
    pub fn set_tick_callback(&self, callback: Box<dyn TickCallback>) {
        struct Adapter(Box<dyn TickCallback>);
        impl TickListener for Adapter {
            fn on_tick(&self, tick: u64) {
                self.0.on_tick(tick);
            }
        }
        self.events.set_tick_listener(Arc::new(Adapter(callback)));
    }

    pub fn clear_tick_callback(&self) {
        self.events.clear_tick_listener();
    }

    /// Snapshot of the dispatch tick.
    pub fn get_tick(&self) -> u64 {
        self.events.tick()
    }

    // ── Pending side-effect drains (iOS) ──
    // Same semantics as the wasm/napi sides. Swift drains these per
    // tick + emits UIKit / SwiftUI side-effects (UNNotification, toast).

    pub fn take_pending_toasts_json(&self) -> String {
        let toasts = self.runtime.state.write().take_pending_toasts();
        serde_json::to_string(&toasts).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn take_pending_browser_notifications_json(&self) -> String {
        let notifs = self.runtime.state.write().take_pending_browser_notifications();
        serde_json::to_string(&notifs).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn take_pending_refetch_ticket_slugs_json(&self) -> String {
        let slugs = self.runtime.state.write().take_pending_refetch_ticket_slugs();
        serde_json::to_string(&slugs).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn take_pending_refetch_pod_keys_json(&self) -> String {
        let keys = self.runtime.state.write().take_pending_refetch_pod_keys();
        serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string())
    }

    // ── State selectors for Swift ──
    // Read-only JSON snapshots. Swift parses into typed view models.

    pub fn pods_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().pods.pods())
            .unwrap_or_else(|_| "[]".to_string())
    }

    /// Strongly-typed pod selector over the shared runtime.state — read by the
    /// tick-driven `PodListReducer` after each realtime dispatch. Returns the
    /// same `PodDto` shape `list_pods` yields, so the reducer can swap its
    /// state without a JSON round-trip or shape mismatch.
    pub fn pods_dto(&self) -> Vec<crate::dto::PodDto> {
        self.runtime
            .state
            .read()
            .pods
            .pods()
            .iter()
            .cloned()
            .map(crate::dto::PodDto::from)
            .collect()
    }

    pub fn channels_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().channels.get_channels())
            .unwrap_or_else(|_| "[]".to_string())
    }

    pub fn tickets_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().tickets.get_tickets())
            .unwrap_or_else(|_| "[]".to_string())
    }

    pub fn runners_json(&self) -> String {
        serde_json::to_string(self.runtime.state.read().runners.runners())
            .unwrap_or_else(|_| "[]".to_string())
    }

    /// Clear all org-scoped state on org switch. Preserves the events
    /// connection + callback registration.
    pub fn reset_for_org_switch(&self) {
        self.runtime.state.write().reset_for_org_switch();
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl AgentsMeshCore {
    /// Connect the realtime events stream. Idempotent — second call is
    /// a no-op while already connected. Swift calls this when the scene
    /// enters foreground.
    pub async fn events_connect(&self) {
        self.events.connect().await;
    }

    /// Disconnect the realtime events stream. Idempotent. Swift calls
    /// this on background to release the socket.
    pub async fn events_disconnect(&self) {
        self.events.disconnect().await;
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl AgentsMeshCore {
    /// Hydrate the auth state by reading storage, refreshing the token if
    /// near expiry, and validating identity against the server. Returns
    /// a strongly-typed `BootstrapResultDto` enum — Swift/Kotlin pattern-
    /// match on `.anonymous` / `.authenticated(user, currentOrg)` /
    /// `.anonymousAfterCleanup(reason)` to drive UI without parsing JSON.
    pub async fn bootstrap(&self) -> BootstrapResultDto {
        BootstrapResultDto::from(self.auth.bootstrap().await)
    }
}

/// Bootstrap the global tracing subscriber. Pass `log_dir = None` for
/// stderr-only output (tests, early boot before sandbox path is known);
/// otherwise we create the directory and roll daily files inside it.
/// Idempotent — Swift can call this from every app launch without guarding.
#[uniffi::export]
pub fn init_logger(log_dir: Option<String>, level: String) -> Result<(), CoreError> {
    let cfg = match log_dir {
        Some(p) => agentsmesh_logging::LogConfig::file(p, level),
        None => agentsmesh_logging::LogConfig::console(level),
    };
    agentsmesh_logging::init(cfg).map_err(|e| CoreError::Unknown {
        message: e.to_string(),
    })?;
    agentsmesh_logging::install_panic_hook();
    Ok(())
}

/// Host-side log entrypoint for Swift/Kotlin callers. Re-emits through the
/// same `tracing` subscriber the Rust workspace uses, so iOS-side events
/// land in the same rolling file as Rust `tracing::*` calls.
#[uniffi::export]
pub fn log_event(level: String, target: String, msg: String) {
    agentsmesh_logging::log_event(&level, &target, &msg);
}
