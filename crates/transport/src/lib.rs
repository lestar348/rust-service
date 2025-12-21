//! Transport implementations for different backends.

pub use service_core::Transport;

#[cfg(feature = "transport_ble")]
pub mod ble;
#[cfg(feature = "transport_http")]
pub mod http;
#[cfg(feature = "transport_mock")]
pub mod mock;
