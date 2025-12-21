use crate::{error::Result, transport::Transport, types::TransportId};

/// API for managing transports at runtime.
pub trait TransportManagerApi {
    fn active_transport(&self) -> Option<TransportId>;
    fn set_transport(&mut self, transport: TransportId) -> Result<()>;
}

/// Placeholder manager implementation.
pub struct TransportManager;

impl TransportManager {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportManagerApi for TransportManager {
    fn active_transport(&self) -> Option<TransportId> {
        None
    }

    fn set_transport(&mut self, _transport: TransportId) -> Result<()> {
        Ok(())
    }
}

impl Transport for TransportManager {
    fn id(&self) -> TransportId {
        "manager".to_string()
    }

    fn start(&self) -> Result<()> {
        Ok(())
    }
}
