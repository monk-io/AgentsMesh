use std::sync::Arc;

use agentsmesh_protocol::MsgType;
use futures::channel::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayStatus {
    Connecting,
    Connected,
    Disconnected,
    Error,
}

impl std::fmt::Display for RelayStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connecting => write!(f, "connecting"),
            Self::Connected => write!(f, "connected"),
            Self::Disconnected => write!(f, "disconnected"),
            Self::Error => write!(f, "error"),
        }
    }
}

pub type OutputCallback = Arc<dyn Fn(Vec<u8>) + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayStatusInfo {
    pub status: RelayStatus,
    pub runner_disconnected: bool,
}

/// Driver-owned, pool-readable status mirror. The driver task is the single
/// writer (under its own lock); the pool's `get_status` / `is_runner_disconnected`
/// / `get_pod_size` read it directly instead of round-tripping a command.
#[derive(Debug, Clone)]
pub(crate) struct StatusSnapshot {
    pub status: RelayStatus,
    pub runner_disconnected: bool,
    pub pod_size: Option<(u16, u16)>,
}

impl Default for StatusSnapshot {
    fn default() -> Self {
        Self {
            status: RelayStatus::Disconnected,
            runner_disconnected: false,
            pod_size: None,
        }
    }
}

pub type StatusCallback = Arc<dyn Fn(RelayStatusInfo) + Send + Sync>;
pub type AcpCallback = Arc<dyn Fn(MsgType, serde_json::Value) + Send + Sync>;
// Fired once when a pod connection is fully torn down (disconnect_inner) so
// adapters can drop their register-once guard and re-register listeners on the
// next subscribe. Carries the pod_key.
pub type DisconnectCallback = Arc<dyn Fn(String) + Send + Sync>;

pub struct ConnectionHandle {
    pub pod_key: String,
    pub subscription_id: String,
    cmd_tx: mpsc::UnboundedSender<crate::command::Command>,
    unsubscribe_tx: mpsc::UnboundedSender<(String, String)>,
}

impl ConnectionHandle {
    pub(crate) fn new(
        pod_key: String,
        subscription_id: String,
        cmd_tx: mpsc::UnboundedSender<crate::command::Command>,
        unsubscribe_tx: mpsc::UnboundedSender<(String, String)>,
    ) -> Self {
        Self {
            pod_key,
            subscription_id,
            cmd_tx,
            unsubscribe_tx,
        }
    }

    pub fn send(&self, data: Vec<u8>) {
        let _ = self.cmd_tx.unbounded_send(crate::command::Command::Send {
            data: String::from_utf8_lossy(&data).into_owned(),
        });
    }

    pub fn unsubscribe(&self) {
        let _ = self
            .unsubscribe_tx
            .unbounded_send((self.pod_key.clone(), self.subscription_id.clone()));
    }
}
