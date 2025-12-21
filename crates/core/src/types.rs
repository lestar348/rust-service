/// Identifier for transport implementations.
pub type TransportId = String;

/// Identifier for features.
pub type FeatureId = String;

/// Abstraction over a clock source.
pub trait Clock: Send + Sync {
    fn now_rfc3339(&self) -> String;
}

/// System-backed clock implementation.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_rfc3339(&self) -> String {
        let seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        format!("{seconds}T00:00:00Z")
    }
}
