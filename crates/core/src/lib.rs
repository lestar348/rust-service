//! Core abstractions for the service project.

pub mod config;
pub mod error;
pub mod event;
pub mod feature;
pub mod manager;
pub mod router;
pub mod transport;
pub mod types;

pub use config::AppConfig;
pub use error::{Error, Result};
pub use event::{
    EventBus, EventError, EventPublisher, EventSubscriber, EventSubscription, TransportEvent,
};
pub use feature::{Feature, FeatureContext, FeatureFuture, FeatureInitError, FeatureResult};
pub use manager::{TransportManager, TransportManagerApi};
pub use router::{
    rpc_handler, InMemoryRouter, RouterError, RpcError, RpcHandler, RpcRegistry, RpcRequest,
    RpcResponse,
};
pub use transport::Transport;
pub use types::{Clock, FeatureId, SystemClock, TransportId};
