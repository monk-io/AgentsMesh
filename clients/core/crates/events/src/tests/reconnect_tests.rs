use crate::reconnect::ReconnectPolicy;

#[test]
fn first_delay_near_initial() {
    let mut policy = ReconnectPolicy::new(1000, 30000);
    let d = policy.next_delay();
    assert!(d.as_millis() >= 1000 && d.as_millis() < 2100);
}

#[test]
fn exponential_backoff_base_growth() {
    let mut policy = ReconnectPolicy::new(1000, 1_000_000);
    let d1 = policy.next_delay(); // 1000 * 2^0 + jitter
    let d2 = policy.next_delay(); // 1000 * 2^1 + jitter
    let d3 = policy.next_delay(); // 1000 * 2^2 + jitter
    assert!(d1.as_millis() >= 1000 && d1.as_millis() < 2100);
    assert!(d2.as_millis() >= 2000 && d2.as_millis() < 3100);
    assert!(d3.as_millis() >= 4000 && d3.as_millis() < 5100);
}

#[test]
fn delay_capped_at_max() {
    let mut policy = ReconnectPolicy::new(1000, 5000);
    for _ in 0..15 {
        assert!(policy.next_delay().as_millis() <= 5000, "delay exceeded max");
    }
}

#[test]
fn never_exhausts() {
    // The old policy returned None past max_attempts and connection_loop gave
    // up → the 永久死亡 bug. A realtime stream must back off forever.
    let mut policy = ReconnectPolicy::new(1000, 30000);
    for _ in 0..1000 {
        assert!(policy.next_delay().as_millis() <= 30000);
    }
}

#[test]
fn reset_returns_to_initial_backoff() {
    let mut policy = ReconnectPolicy::new(1000, 1_000_000);
    for _ in 0..5 {
        let _ = policy.next_delay();
    }
    policy.reset();
    assert_eq!(policy.attempts(), 0);
    // After reset the next delay is back near initial, not the escalated level.
    assert!(policy.next_delay().as_millis() < 2100);
}

#[test]
fn zero_base_delay_is_jitter_only() {
    let mut policy = ReconnectPolicy::new(0, 30000);
    assert!(policy.next_delay().as_millis() < 1000);
}
