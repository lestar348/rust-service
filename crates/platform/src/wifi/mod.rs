use service_core::{Result, TransportId};

pub struct WifiDetector;

impl WifiDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self) -> Option<TransportId> {
        None
    }
}

pub mod detector;
pub mod linux;
pub mod mock;
