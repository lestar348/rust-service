use std::sync::Arc;

use service_core::{
    event::{
        EventBus, EventError, EventPublisher, EventSubscriber, EventSubscription, TransportEvent,
    },
    router::{InMemoryRouter, RpcError, RpcRegistry, RpcRequest, RpcResponse},
    transport::Transport,
    types::TransportId,
};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct MockTransport {
    registry: InMemoryRouter,
    events: BroadcastEventBus,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            registry: InMemoryRouter::new(),
            events: BroadcastEventBus::new(),
        }
    }

    pub fn registry(&self) -> Arc<dyn RpcRegistry> {
        Arc::new(self.registry.clone())
    }

    pub fn events(&self) -> Arc<dyn EventBus> {
        Arc::new(self.events.clone())
    }

    pub async fn handle_incoming(&self, req: RpcRequest) -> Result<RpcResponse, RpcError> {
        self.registry.dispatch(req).await
    }
}

impl Transport for MockTransport {
    fn id(&self) -> TransportId {
        "mock".to_string()
    }

    fn start(&self) -> service_core::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct BroadcastEventBus {
    sender: broadcast::Sender<TransportEvent>,
}

impl BroadcastEventBus {
    fn new() -> Self {
        let (sender, _) = broadcast::channel(16);
        Self { sender }
    }
}

impl EventPublisher for BroadcastEventBus {
    fn publish(&self, event: TransportEvent) -> Result<(), EventError> {
        self.sender
            .send(event)
            .map(|_| ())
            .map_err(|err| EventError::Publish(err.to_string()))
    }
}

impl EventSubscriber for BroadcastEventBus {
    fn subscribe(&self) -> EventSubscription {
        EventSubscription::new(self.sender.subscribe())
    }
}
