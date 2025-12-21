use service_core::{Transport, TransportId};

#[allow(dead_code)]
pub struct MockTransport;

impl MockTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for MockTransport {
    fn id(&self) -> TransportId {
        "mock".to_string()
    }

    fn start(&self) -> service_core::Result<()> {
        Ok(())
    }
}
