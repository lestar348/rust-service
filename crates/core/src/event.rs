use std::{
    error::Error,
    fmt::Display,
    sync::{mpsc, Arc},
};

/// Event payload emitted over transports.
#[derive(Debug, Clone)]
pub struct TransportEvent {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum EventError {
    Publish(String),
    Receive(String),
}

impl Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventError::Publish(msg) | EventError::Receive(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for EventError {}

/// An owned subscription to transport events.
pub struct EventSubscription {
    receiver: mpsc::Receiver<TransportEvent>,
}

impl EventSubscription {
    pub fn new(receiver: mpsc::Receiver<TransportEvent>) -> Self {
        Self { receiver }
    }

    pub async fn recv(&mut self) -> Result<TransportEvent, EventError> {
        self.receiver
            .recv()
            .map_err(|err| EventError::Receive(err.to_string()))
    }
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: TransportEvent) -> Result<(), EventError>;
}

pub trait EventSubscriber: Send + Sync {
    fn subscribe(&self) -> EventSubscription;
}

/// Convenience trait for objects that support both publishing and subscribing.
pub trait EventBus: EventPublisher + EventSubscriber {}

impl<T> EventBus for T where T: EventPublisher + EventSubscriber {}

/// Shared type alias for an event bus trait object.
pub type DynEventBus = Arc<dyn EventBus>;
