use std::time::Duration;

use tokio::sync::mpsc;

use crate::heartbeat::{HeartbeatEvent, HeartbeatManager};

#[tokio::test]
async fn test_ping_sent_after_interval() {
    let mut hb = HeartbeatManager::new(80, 5000);
    let (tx, mut rx) = mpsc::channel(16);
    hb.start(tx);

    match tokio::time::timeout(Duration::from_millis(300), rx.recv()).await {
        Ok(Some(HeartbeatEvent::SendPing)) => {}
        other => panic!("expected SendPing, got timeout={}", other.is_err()),
    }
    hb.stop();
}

#[tokio::test]
async fn test_pong_timeout_fires_when_no_pong() {
    let mut hb = HeartbeatManager::new(200, 50);
    let (tx, mut rx) = mpsc::channel(16);
    hb.start(tx);

    match tokio::time::timeout(Duration::from_millis(500), rx.recv()).await {
        Ok(Some(HeartbeatEvent::SendPing)) => {}
        _ => panic!("expected SendPing first"),
    }
    match tokio::time::timeout(Duration::from_millis(300), rx.recv()).await {
        Ok(Some(HeartbeatEvent::PongTimeout)) => {}
        other => panic!("expected PongTimeout, got timeout={}", other.is_err()),
    }
}

#[tokio::test]
async fn test_pong_received_prevents_timeout() {
    let mut hb = HeartbeatManager::new(50, 100);
    let (tx, mut rx) = mpsc::channel(16);
    hb.start(tx);

    // consume first SendPing
    let _ = rx.recv().await;
    hb.pong_received();

    // next event should be another SendPing, not PongTimeout
    match tokio::time::timeout(Duration::from_millis(200), rx.recv()).await {
        Ok(Some(HeartbeatEvent::SendPing)) => {}
        Ok(Some(HeartbeatEvent::PongTimeout)) => panic!("got PongTimeout after pong_received"),
        other => panic!("unexpected: timeout={}", other.is_err()),
    }
    hb.stop();
}

#[tokio::test]
async fn test_multiple_ping_cycles() {
    let mut hb = HeartbeatManager::new(40, 5000);
    let (tx, mut rx) = mpsc::channel(16);
    hb.start(tx);

    for _ in 0..3 {
        match tokio::time::timeout(Duration::from_millis(200), rx.recv()).await {
            Ok(Some(HeartbeatEvent::SendPing)) => hb.pong_received(),
            other => panic!("expected SendPing, got timeout={}", other.is_err()),
        }
    }
    hb.stop();
}

#[tokio::test]
async fn test_stop_terminates_loop() {
    let mut hb = HeartbeatManager::new(30, 30);
    let (tx, mut rx) = mpsc::channel(16);
    hb.start(tx);
    hb.stop();

    tokio::time::sleep(Duration::from_millis(100)).await;
    // channel should be closed or empty
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn test_restart_after_stop() {
    let mut hb = HeartbeatManager::new(50, 5000);
    let (tx1, _rx1) = mpsc::channel(16);
    hb.start(tx1);
    hb.stop();

    let (tx2, mut rx2) = mpsc::channel(16);
    hb.start(tx2);

    match tokio::time::timeout(Duration::from_millis(200), rx2.recv()).await {
        Ok(Some(HeartbeatEvent::SendPing)) => {}
        other => panic!("expected SendPing after restart, got timeout={}", other.is_err()),
    }
    hb.stop();
}

#[tokio::test]
async fn test_drop_stops_heartbeat() {
    let (tx, mut rx) = mpsc::channel(16);
    {
        let mut hb = HeartbeatManager::new(30, 5000);
        hb.start(tx);
    } // hb dropped here

    tokio::time::sleep(Duration::from_millis(100)).await;
    // may get one ping but should not infinite-loop
    let count = std::iter::from_fn(|| rx.try_recv().ok()).count();
    assert!(count <= 1);
}
