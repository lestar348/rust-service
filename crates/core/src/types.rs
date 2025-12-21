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
        time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
    }
}
