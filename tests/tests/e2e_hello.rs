use std::sync::Arc;

use service_core::{feature::FeatureContext, router::RpcRequest, types::Clock, Feature};
use service_features::hello_world::{api, HelloWorldFeature};
use service_transport::mock::MockTransport;

struct FixedClock {
    now: String,
}

impl FixedClock {
    fn new(now: &str) -> Self {
        Self {
            now: now.to_string(),
        }
    }
}

impl Clock for FixedClock {
    fn now_rfc3339(&self) -> String {
        self.now.clone()
    }
}

#[tokio::test]
async fn hello_world_rpc_flow() {
    let transport = MockTransport::new();
    let registry = transport.registry();
    let events = transport.events();
    let clock = Arc::new(FixedClock::new("2025-12-21T00:00:00Z"));

    let feature = HelloWorldFeature::with_clock(clock.clone());
    feature
        .init(FeatureContext::new(registry.clone(), events.clone(), clock))
        .await
        .expect("feature init");

    let mut subscription = events.subscribe();
    let request = RpcRequest::new(api::SERVICE, api::METHOD_GET, Vec::new(), 1_000);
    let response = transport
        .handle_incoming(request)
        .await
        .expect("rpc response");

    let message = String::from_utf8(response.payload).expect("utf-8 response");
    assert_eq!(message, "2025-12-21T00:00:00Z hello world");

    let event = subscription.recv().await.expect("event received");
    assert_eq!(event.topic, "hello/called");
}
