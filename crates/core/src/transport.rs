use crate::{error::Result, types::TransportId};

/// Transport abstraction over different backends (mock, HTTP, BLE).
pub trait Transport {
    fn id(&self) -> TransportId;
    fn start(&self) -> Result<()>;
}
