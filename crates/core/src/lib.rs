//! Core abstractions for the service project.

pub mod config;
pub mod error;
pub mod feature;
pub mod manager;
pub mod transport;
pub mod types;

pub use config::AppConfig;
pub use error::{Error, Result};
pub use feature::Feature;
pub use manager::{TransportManager, TransportManagerApi};
pub use transport::Transport;
pub use types::{FeatureId, TransportId};
