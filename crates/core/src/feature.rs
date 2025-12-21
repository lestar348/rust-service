use crate::{error::Result, transport::Transport, types::FeatureId};

/// Trait implemented by all feature modules.
pub trait Feature {
    fn id(&self) -> FeatureId;
    fn initialize(&self, _transport: &dyn Transport) -> Result<()> {
        Ok(())
    }
}
