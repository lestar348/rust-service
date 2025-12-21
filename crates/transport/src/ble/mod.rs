use service_core::{Result, Transport, TransportId};

#[allow(dead_code)]
pub struct BleTransport;

impl BleTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for BleTransport {
    fn id(&self) -> TransportId {
        "ble".to_string()
    }

    fn start(&self) -> Result<()> {
        Ok(())
    }
}
