//! End-to-end integration tests: drive the real `RelayConnectionPool` against a
//! mock relay WebSocket server (tokio-tungstenite) through the native transport.
//! Unlike the unit tests (which poke `ConnectionState` directly), these exercise
//! connect → codec → dispatch → callbacks → reconnect over a real socket.

use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use agentsmesh_protocol::{encode_message, MsgType};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex as TokioMutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::pool::RelayConnectionPool;
use crate::types::{OutputCallback, RelayStatus};

type ServerSink = SplitSink<WebSocketStream<TcpStream>, Message>;

/// Scriptable mock relay. Accepts (re-)connections; `to_client` frames are sent
/// to the current connection, inbound frames land in `from_client`, `drop` closes
/// the active socket (to exercise reconnect). `conn_count` counts accepted sockets.
struct MockRelay {
    url: String,
    to_client: UnboundedSender<Vec<u8>>,
    from_client: TokioMutex<UnboundedReceiver<Vec<u8>>>,
    drop_signal: UnboundedSender<()>,
    conn_count: Arc<AtomicUsize>,
}

async fn start_mock_relay() -> MockRelay {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let (to_client, mut to_client_rx) = unbounded_channel::<Vec<u8>>();
    let (from_client_tx, from_client_rx) = unbounded_channel::<Vec<u8>>();
    let (drop_signal, mut drop_rx) = unbounded_channel::<()>();
    let conn_count = Arc::new(AtomicUsize::new(0));

    let active: Arc<TokioMutex<Option<ServerSink>>> = Arc::new(TokioMutex::new(None));

    // Drain test → current socket.
    {
        let active = active.clone();
        tokio::spawn(async move {
            while let Some(frame) = to_client_rx.recv().await {
                if let Some(sink) = active.lock().await.as_mut() {
                    let _ = sink.send(Message::Binary(frame.into())).await;
                }
            }
        });
    }
    // Drop signal → close current socket.
    {
        let active = active.clone();
        tokio::spawn(async move {
            while drop_rx.recv().await.is_some() {
                if let Some(sink) = active.lock().await.as_mut() {
                    let _ = sink.send(Message::Close(None)).await;
                }
            }
        });
    }
    // Accept loop: swap active sink BEFORE bumping conn_count so a >=N observation
    // guarantees the sink is ready (avoids a send-before-swap race in reconnect).
    {
        let active = active.clone();
        let cc = conn_count.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else { break };
                let ws = match tokio_tungstenite::accept_async(stream).await {
                    Ok(w) => w,
                    Err(_) => continue,
                };
                let (write, mut read) = ws.split();
                *active.lock().await = Some(write);
                cc.fetch_add(1, SeqCst);
                let ftx = from_client_tx.clone();
                tokio::spawn(async move {
                    while let Some(Ok(msg)) = read.next().await {
                        if let Message::Binary(b) = msg {
                            let _ = ftx.send(b.to_vec());
                        }
                    }
                });
            }
        });
    }

    MockRelay {
        url,
        to_client,
        from_client: TokioMutex::new(from_client_rx),
        drop_signal,
        conn_count,
    }
}

impl MockRelay {
    fn push(&self, msg_type: MsgType, payload: &[u8]) {
        self.to_client.send(encode_message(msg_type, payload)).unwrap();
    }
    async fn recv(&self, timeout: Duration) -> Option<Vec<u8>> {
        let mut rx = self.from_client.lock().await;
        tokio::time::timeout(timeout, rx.recv()).await.ok().flatten()
    }
}

fn make_output_cb() -> (OutputCallback, Arc<Mutex<Vec<Vec<u8>>>>) {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let r = buf.clone();
    let cb: OutputCallback = Arc::new(move |data| r.lock().unwrap().push(data));
    (cb, buf)
}

async fn wait_until<F: Fn() -> bool>(f: F, timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    while !f() {
        if start.elapsed() > timeout {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    true
}

async fn wait_connected(pool: &RelayConnectionPool, pod: &str) -> bool {
    let start = std::time::Instant::now();
    while pool.get_status(pod).await != RelayStatus::Connected {
        if start.elapsed() > Duration::from_secs(3) {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    true
}

fn buf_has(buf: &Arc<Mutex<Vec<Vec<u8>>>>, needle: &[u8]) -> bool {
    buf.lock().unwrap().iter().any(|f| f == needle)
}

#[tokio::test]
async fn output_frame_reaches_subscriber() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    mock.push(MsgType::Output, b"hello-terminal");
    assert!(
        wait_until(|| buf_has(&buf, b"hello-terminal"), Duration::from_secs(3)).await,
        "output frame did not reach subscriber callback",
    );
}

#[tokio::test]
async fn input_send_reaches_server() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, _buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    pool.send("pod-1", "echo-me").await;
    let frame = mock.recv(Duration::from_secs(3)).await.expect("no input frame");
    assert_eq!(frame[0], MsgType::Input as u8, "wrong frame type");
    assert_eq!(&frame[1..], b"echo-me");
}

#[tokio::test]
async fn resize_debounced_reaches_server() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, _buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    pool.send_resize("pod-1", 120, 40).await; // debounced ~150ms
    let frame = mock.recv(Duration::from_secs(3)).await.expect("no resize frame");
    assert_eq!(frame[0], MsgType::Resize as u8, "wrong frame type");
    // 4-byte big-endian cols,rows payload
    assert_eq!(&frame[1..], &[0, 120, 0, 40]);
}

#[tokio::test]
async fn snapshot_replays_to_subscriber() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    let snap = serde_json::json!({"serialized_content":"RESTORED-STATE","cols":80,"rows":24});
    mock.push(MsgType::Snapshot, snap.to_string().as_bytes());
    assert!(
        wait_until(|| buf_has(&buf, b"RESTORED-STATE"), Duration::from_secs(3)).await,
        "snapshot content not replayed to subscriber",
    );
    assert!(buf_has(&buf, crate::dispatch::ANSI_CLEAR), "snapshot did not clear screen first");
}

#[tokio::test]
async fn runner_disconnect_then_reconnect_toggles_flag() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, _buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    mock.push(MsgType::RunnerDisconnected, &[]);
    let start = std::time::Instant::now();
    while !pool.is_runner_disconnected("pod-1").await {
        assert!(start.elapsed() < Duration::from_secs(3), "runner_disconnected never set");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    mock.push(MsgType::RunnerReconnected, &[]);
    let start = std::time::Instant::now();
    while pool.is_runner_disconnected("pod-1").await {
        assert!(start.elapsed() < Duration::from_secs(3), "runner_disconnected never cleared");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn acp_command_out_and_event_in() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, _buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_connected(&pool, "pod-1").await, "never connected");

    let acp_buf = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));
    {
        let b = acp_buf.clone();
        pool.on_acp_message(
            "pod-1",
            Arc::new(move |_mt, val| b.lock().unwrap().push(val)),
        )
        .await;
    }

    pool.send_acp_command("pod-1", &serde_json::json!({"cmd":"go"}))
        .await
        .unwrap();
    let frame = mock.recv(Duration::from_secs(3)).await.expect("no acp command frame");
    assert_eq!(frame[0], MsgType::AcpCommand as u8, "wrong frame type");

    mock.push(MsgType::AcpEvent, serde_json::json!({"event":"started"}).to_string().as_bytes());
    assert!(
        wait_until(|| !acp_buf.lock().unwrap().is_empty(), Duration::from_secs(3)).await,
        "acp event did not reach on_acp_message callback",
    );
}

#[tokio::test]
async fn reconnects_after_server_drop() {
    let mock = start_mock_relay().await;
    let (pool, _rx) = RelayConnectionPool::new();
    let (cb, buf) = make_output_cb();
    pool.subscribe("pod-1", "sub-1", &mock.url, "tok", cb).await;
    assert!(wait_until(|| mock.conn_count.load(SeqCst) >= 1, Duration::from_secs(10)).await, "no first connect");

    mock.drop_signal.send(()).unwrap();
    // schedule_reconnect waits ~BASE_RECONNECT_DELAY_MS (1s) before re-dialing.
    // Timeouts are generous: under the test binary's full parallelism (one
    // tokio runtime per #[tokio::test] thread) the reconnect's wall clock can
    // stretch well past the ~1s backoff. This asserts the behavior, not an SLA.
    assert!(
        wait_until(|| mock.conn_count.load(SeqCst) >= 2, Duration::from_secs(20)).await,
        "pool did not reconnect after server drop",
    );

    mock.push(MsgType::Output, b"after-reconnect");
    assert!(
        wait_until(|| buf_has(&buf, b"after-reconnect"), Duration::from_secs(10)).await,
        "output did not flow after reconnect",
    );
}
