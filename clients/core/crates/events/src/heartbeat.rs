use std::time::Duration;

use agentsmesh_transport::runtime::{PlatformRuntime, Runtime, TaskHandle};
use tokio::sync::mpsc;
use tracing::{debug, warn};

pub(crate) enum HeartbeatCommand {
    PongReceived,
    Stop,
}

pub(crate) enum HeartbeatEvent {
    SendPing,
    PongTimeout,
}

pub(crate) struct HeartbeatManager<R: Runtime = PlatformRuntime> {
    ping_interval: Duration,
    pong_timeout: Duration,
    cmd_tx: Option<mpsc::Sender<HeartbeatCommand>>,
    task: Option<R::TaskHandle>,
    runtime: R,
}

impl HeartbeatManager<PlatformRuntime> {
    #[allow(dead_code)]
    pub fn new(ping_interval_ms: u64, pong_timeout_ms: u64) -> Self {
        Self::with_runtime(PlatformRuntime, ping_interval_ms, pong_timeout_ms)
    }
}

impl<R: Runtime> HeartbeatManager<R> {
    pub fn with_runtime(runtime: R, ping_interval_ms: u64, pong_timeout_ms: u64) -> Self {
        Self {
            ping_interval: Duration::from_millis(ping_interval_ms),
            pong_timeout: Duration::from_millis(pong_timeout_ms),
            cmd_tx: None,
            task: None,
            runtime,
        }
    }

    pub fn start(&mut self, event_tx: mpsc::Sender<HeartbeatEvent>) {
        self.stop();
        let (cmd_tx, cmd_rx) = mpsc::channel(16);
        self.cmd_tx = Some(cmd_tx);

        let ping_interval = self.ping_interval;
        let pong_timeout = self.pong_timeout;
        let rt = self.runtime.clone();

        self.task = Some(self.runtime.spawn(Box::pin(
            heartbeat_loop(rt, ping_interval, pong_timeout, cmd_rx, event_tx),
        )));
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.cmd_tx.take() {
            let _ = tx.try_send(HeartbeatCommand::Stop);
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }

    pub fn pong_received(&self) {
        if let Some(tx) = &self.cmd_tx {
            let _ = tx.try_send(HeartbeatCommand::PongReceived);
        }
    }
}

impl<R: Runtime> Drop for HeartbeatManager<R> {
    fn drop(&mut self) {
        self.stop();
    }
}

async fn heartbeat_loop<R: Runtime>(
    runtime: R,
    ping_interval: Duration,
    pong_timeout: Duration,
    mut cmd_rx: mpsc::Receiver<HeartbeatCommand>,
    event_tx: mpsc::Sender<HeartbeatEvent>,
) {
    runtime.sleep(ping_interval).await;

    loop {
        if event_tx.send(HeartbeatEvent::SendPing).await.is_err() {
            break;
        }
        debug!("heartbeat: sending ping");

        // Wait for pong with timeout
        tokio::select! {
            _ = runtime.sleep(pong_timeout) => {
                warn!("heartbeat: pong timeout");
                let _ = event_tx.send(HeartbeatEvent::PongTimeout).await;
                break;
            }
            cmd = cmd_rx.recv() => {
                match cmd {
                    Some(HeartbeatCommand::PongReceived) => {
                        debug!("heartbeat: pong received");
                    }
                    Some(HeartbeatCommand::Stop) | None => break,
                }
            }
        }

        // Wait for next ping interval, watching for stop
        tokio::select! {
            _ = runtime.sleep(ping_interval) => {}
            cmd = cmd_rx.recv() => {
                match cmd {
                    Some(HeartbeatCommand::Stop) | None => break,
                    Some(HeartbeatCommand::PongReceived) => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat_sends_ping() {
        let mut hb = HeartbeatManager::new(100, 5000);
        let (event_tx, mut event_rx) = mpsc::channel(16);
        hb.start(event_tx);

        match tokio::time::timeout(Duration::from_millis(300), event_rx.recv()).await {
            Ok(Some(HeartbeatEvent::SendPing)) => {}
            other => panic!("expected SendPing, got {:?}", other.is_ok()),
        }
        hb.stop();
    }

    #[tokio::test]
    async fn test_heartbeat_pong_timeout() {
        let mut hb = HeartbeatManager::new(200, 50);
        let (event_tx, mut event_rx) = mpsc::channel(16);
        hb.start(event_tx);

        match tokio::time::timeout(Duration::from_millis(500), event_rx.recv()).await {
            Ok(Some(HeartbeatEvent::SendPing)) => {}
            _ => panic!("expected SendPing first"),
        }
        match tokio::time::timeout(Duration::from_millis(300), event_rx.recv()).await {
            Ok(Some(HeartbeatEvent::PongTimeout)) => {}
            other => panic!("expected PongTimeout, got {:?}", other.is_ok()),
        }
    }

    #[tokio::test]
    async fn test_heartbeat_pong_clears_timeout() {
        let mut hb = HeartbeatManager::new(50, 80);
        let (event_tx, mut event_rx) = mpsc::channel(16);
        hb.start(event_tx);

        let _ = event_rx.recv().await; // SendPing
        hb.pong_received();

        match tokio::time::timeout(Duration::from_millis(200), event_rx.recv()).await {
            Ok(Some(HeartbeatEvent::SendPing)) => {}
            other => panic!("expected second SendPing, got {:?}", other.is_ok()),
        }
        hb.stop();
    }

    #[tokio::test]
    async fn test_heartbeat_stop() {
        let mut hb = HeartbeatManager::new(50, 50);
        let (event_tx, mut event_rx) = mpsc::channel(16);
        hb.start(event_tx);
        hb.stop();

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(event_rx.try_recv().is_err());
    }
}
