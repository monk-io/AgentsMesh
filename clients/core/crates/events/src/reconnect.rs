use std::time::Duration;

pub struct ReconnectPolicy {
    initial_delay_ms: u64,
    max_delay_ms: u64,
    attempts: u32,
}

impl ReconnectPolicy {
    pub fn new(initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self {
            initial_delay_ms,
            max_delay_ms,
            attempts: 0,
        }
    }

    /// Next backoff delay. Never exhausts — a realtime stream must reconnect
    /// indefinitely; `attempts` only escalates the (capped) delay. Reset on a
    /// data-ready session so one healthy connection clears the failure count.
    pub fn next_delay(&mut self) -> Duration {
        self.attempts += 1;
        let base = self.initial_delay_ms as f64 * 2_f64.powi(self.attempts as i32 - 1);
        let jitter = rand_jitter();
        let delay_ms = (base + jitter).min(self.max_delay_ms as f64) as u64;
        Duration::from_millis(delay_ms)
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

fn rand_jitter() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use web_time::SystemTime;

    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    let hash = hasher.finish();
    (hash % 1000) as f64
}
