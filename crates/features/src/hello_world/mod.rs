use std::sync::Arc;

use service_core::{
    feature::{FeatureFuture, FeatureInitError},
    types::Clock,
    Feature, FeatureContext, SystemClock,
};

mod adapter;
pub mod api;
mod domain;

pub struct HelloWorldFeature {
    clock: Arc<dyn Clock>,
}

impl HelloWorldFeature {
    pub fn new() -> Self {
        Self {
            clock: Arc::new(SystemClock),
        }
    }

    pub fn with_clock(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}

impl Feature for HelloWorldFeature {
    fn name(&self) -> &'static str {
        "hello_world"
    }

    fn init(&self, ctx: FeatureContext) -> FeatureFuture<'_> {
        let clock = self.clock.clone();
        Box::pin(async move {
            adapter::register(ctx.router, ctx.events, clock)
                .map_err(|err| FeatureInitError::new(err.to_string()))?;
            Ok(())
        })
    }
}
