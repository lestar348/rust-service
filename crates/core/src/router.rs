use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

/// Request envelope for incoming RPC calls.
#[derive(Debug, Clone)]
pub struct RpcRequest {
    pub service: String,
    pub method: String,
    pub payload: Vec<u8>,
    pub timeout_ms: u64,
}

impl RpcRequest {
    pub fn new(
        service: impl Into<String>,
        method: impl Into<String>,
        payload: Vec<u8>,
        timeout_ms: u64,
    ) -> Self {
        Self {
            service: service.into(),
            method: method.into(),
            payload,
            timeout_ms,
        }
    }
}

/// Successful RPC response envelope.
#[derive(Debug, Clone)]
pub struct RpcResponse {
    pub payload: Vec<u8>,
}

/// RPC level errors surfaced to transports.
#[derive(Debug, Clone)]
pub enum RpcError {
    Decode(String),
    UnknownMethod,
    Internal(String),
}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::Decode(msg) => write!(f, "failed to decode request: {msg}"),
            RpcError::UnknownMethod => write!(f, "unknown service or method"),
            RpcError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl Error for RpcError {}

/// Errors emitted by the router during registration.
#[derive(Debug, Clone)]
pub enum RouterError {
    DuplicateRegistration { service: String, method: String },
    InvalidName,
}

impl Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterError::DuplicateRegistration { service, method } => {
                write!(f, "handler already registered for {service}.{method}")
            }
            RouterError::InvalidName => write!(f, "invalid service or method name"),
        }
    }
}

impl Error for RouterError {}

pub type RpcFuture = Pin<Box<dyn Future<Output = Result<RpcResponse, RpcError>> + Send>>;
pub type RpcHandler = Arc<dyn Fn(RpcRequest) -> RpcFuture + Send + Sync>;

/// Abstraction over server-side RPC routing and dispatching.
pub trait RpcRegistry: Send + Sync {
    fn register(&self, service: &str, method: &str, handler: RpcHandler)
        -> Result<(), RouterError>;
    fn dispatch(&self, req: RpcRequest) -> RpcFuture;
}

/// Helper to wrap async closures into an [`RpcHandler`].
pub fn rpc_handler<F, Fut>(func: F) -> RpcHandler
where
    F: Fn(RpcRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<RpcResponse, RpcError>> + Send + 'static,
{
    Arc::new(move |req| Box::pin(func(req)))
}

/// Simple in-memory router implementation that can be embedded in transports.
#[derive(Default, Clone)]
pub struct InMemoryRouter {
    handlers: Arc<Mutex<HashMap<(String, String), RpcHandler>>>,
}

impl InMemoryRouter {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn insert(&self, service: &str, method: &str, handler: RpcHandler) -> Result<(), RouterError> {
        if service.trim().is_empty() || method.trim().is_empty() {
            return Err(RouterError::InvalidName);
        }

        let mut handlers = self.handlers.lock().expect("router mutex poisoned");
        match handlers.entry((service.to_string(), method.to_string())) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(handler);
                Ok(())
            }
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(RouterError::DuplicateRegistration {
                    service: service.to_string(),
                    method: method.to_string(),
                })
            }
        }
    }

    fn get(&self, service: &str, method: &str) -> Option<RpcHandler> {
        let handlers = self.handlers.lock().expect("router mutex poisoned");
        handlers
            .get(&(service.to_string(), method.to_string()))
            .cloned()
    }
}

impl RpcRegistry for InMemoryRouter {
    fn register(
        &self,
        service: &str,
        method: &str,
        handler: RpcHandler,
    ) -> Result<(), RouterError> {
        self.insert(service, method, handler)
    }

    fn dispatch(&self, req: RpcRequest) -> RpcFuture {
        match self.get(&req.service, &req.method) {
            Some(handler) => handler(req),
            None => Box::pin(async { Err(RpcError::UnknownMethod) }),
        }
    }
}
