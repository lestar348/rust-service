use std::sync::{mpsc, Arc, Mutex};

use service_core::{
    event::{
        EventBus, EventError, EventPublisher, EventSubscriber, EventSubscription, TransportEvent,
    },
    router::{InMemoryRouter, RpcError, RpcRequest, RpcResponse, RpcRouter},
    transport::Transport,
    types::TransportId,
};

#[derive(Clone)]
pub struct MockTransport {
    router: InMemoryRouter,
    subscribers: Arc<Mutex<Vec<mpsc::Sender<TransportEvent>>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            router: InMemoryRouter::new(),
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn router(&self) -> Arc<dyn RpcRouter> {
        Arc::new(self.router.clone())
    }

    pub fn events(&self) -> Arc<dyn EventBus> {
        Arc::new(MockEvents {
            subscribers: self.subscribers.clone(),
        })
    }

    pub async fn handle_incoming(&self, req: RpcRequest) -> Result<RpcResponse, RpcError> {
        match self.router.get(&req.service, &req.method) {
            Some(handler) => handler(req).await,
            None => Err(RpcError::UnknownMethod),
        }
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
struct MockEvents {
    subscribers: Arc<Mutex<Vec<mpsc::Sender<TransportEvent>>>>,
}

impl EventPublisher for MockEvents {
    fn publish(&self, event: TransportEvent) -> Result<(), EventError> {
        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|err| EventError::Publish(err.to_string()))?;

        subscribers.retain(|sender| sender.send(event.clone()).is_ok());
        Ok(())
    }
}

impl EventSubscriber for MockEvents {
    fn subscribe(&self) -> EventSubscription {
        let (tx, rx) = mpsc::channel();
        if let Ok(mut subscribers) = self.subscribers.lock() {
            subscribers.push(tx);
        }
        EventSubscription::new(rx)
    }
}
