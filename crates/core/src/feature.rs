use std::{future::Future, pin::Pin, sync::Arc};

use crate::{event::DynEventBus, router::RpcRegistry, types::Clock};

/// Context provided to features during initialization.
#[derive(Clone)]
pub struct FeatureContext {
    pub router: Arc<dyn RpcRegistry>,
    pub events: DynEventBus,
    pub clock: Arc<dyn Clock>,
}

impl FeatureContext {
    pub fn new(router: Arc<dyn RpcRegistry>, events: DynEventBus, clock: Arc<dyn Clock>) -> Self {
        Self {
            router,
            events,
            clock,
        }
    }
}

pub type FeatureFuture<'a> = Pin<Box<dyn Future<Output = FeatureResult<()>> + Send + 'a>>;

pub type FeatureResult<T> = Result<T, FeatureInitError>;

#[derive(Debug)]
pub struct FeatureInitError {
    pub message: String,
}

impl FeatureInitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FeatureInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FeatureInitError {}

/// Trait implemented by all feature modules.
pub trait Feature: Send + Sync {
    fn name(&self) -> &'static str;
    fn init(&self, ctx: FeatureContext) -> FeatureFuture<'_>;
}
