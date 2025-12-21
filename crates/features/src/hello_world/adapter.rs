use std::sync::Arc;

use service_core::{
    event::{EventBus, TransportEvent},
    router::{rpc_handler, RouterError, RpcError, RpcRequest, RpcResponse, RpcRouter},
    types::Clock,
};

use crate::hello_world::{api, domain};

pub fn register(
    router: Arc<dyn RpcRouter>,
    events: Arc<dyn EventBus>,
    clock: Arc<dyn Clock>,
) -> Result<(), RouterError> {
    let handler = rpc_handler(move |_req: RpcRequest| {
        let events = events.clone();
        let clock = clock.clone();
        async move {
            let timestamp = clock.now_rfc3339();
            let message = domain::get_message(&timestamp);

            events
                .publish(TransportEvent {
                    topic: "hello/called".to_string(),
                    payload: Vec::new(),
                })
                .map_err(|err| RpcError::Internal(err.to_string()))?;

            Ok(RpcResponse {
                payload: message.into_bytes(),
            })
        }
    });

    router.register(api::SERVICE, api::METHOD_GET, handler)
}
