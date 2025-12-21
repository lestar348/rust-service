use service_core::{Result, Transport, TransportId};

#[allow(dead_code)]
pub struct HttpTransport;

impl HttpTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for HttpTransport {
    fn id(&self) -> TransportId {
        "http".to_string()
    }

    fn start(&self) -> Result<()> {
        Ok(())
    }
}
