use service_core::{Feature, FeatureId, Transport};

pub struct HelloWorldFeature;

impl HelloWorldFeature {
    pub fn new() -> Self {
        Self
    }
}

impl Feature for HelloWorldFeature {
    fn id(&self) -> FeatureId {
        "hello_world".to_string()
    }

    fn initialize(&self, _transport: &dyn Transport) -> service_core::Result<()> {
        Ok(())
    }
}
