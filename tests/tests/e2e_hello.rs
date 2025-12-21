use std::sync::Arc;

use service_core::{feature::FeatureContext, router::RpcRequest, types::Clock, Feature};
use service_features::hello_world::{api, HelloWorldFeature};
use service_tests::block_on;
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

#[test]
fn hello_world_rpc_flow() {
    block_on(async {
        let transport = MockTransport::new();
        let router = transport.router();
        let events = transport.events();
        let clock = Arc::new(FixedClock::new("2025-12-21T00:00:00Z"));

        let feature = HelloWorldFeature::with_clock(clock.clone());
        feature
            .init(FeatureContext::new(router.clone(), events.clone(), clock))
            .await
            .expect("feature init");

        let mut subscription = events.subscribe();
        let request = RpcRequest::new(api::SERVICE, api::METHOD_GET, Vec::new(), 1_000);
        let response = transport
            .handle_incoming(request)
            .await
            .expect("rpc response");

        let message = String::from_utf8(response.payload).expect("utf-8 response");
        assert!(message.contains("hello world"));
        assert!(
            message.contains('T') || message.contains('Z') || message.contains('+'),
            "expected time separator in message"
        );

        let event = subscription.recv().await.expect("event received");
        assert_eq!(event.topic, "hello/called");
    });
}
