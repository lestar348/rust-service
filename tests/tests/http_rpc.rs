use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use reqwest::StatusCode;
use service_core::{feature::FeatureContext, types::Clock, Feature};
use service_features::hello_world::{api, HelloWorldFeature};
use service_transport::{
    http::{
        protocol::{HttpRpcRequest, HttpRpcResponse},
        HttpServerTransport,
    },
    mock::MockTransport,
};

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
async fn http_rpc_hello_world() {
    let transport = MockTransport::new();
    let registry = transport.registry();
    let events = transport.events();
    let clock = Arc::new(FixedClock::new("2025-12-21T00:00:00Z"));

    let feature = HelloWorldFeature::with_clock(clock.clone());
    feature
        .init(FeatureContext::new(registry.clone(), events, clock))
        .await
        .expect("feature init");

    let server = HttpServerTransport::new(registry);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind listener");
    let addr = listener.local_addr().expect("local addr");

    let server_task = tokio::spawn(async move { server.serve(addr).await });

    let client = reqwest::Client::new();
    let url = format!("http://{}/rpc", addr);
    let request = HttpRpcRequest {
        service: api::SERVICE.to_string(),
        method: api::METHOD_GET.to_string(),
        payload_b64: String::new(),
        timeout_ms: 1_000,
    };

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body: HttpRpcResponse = response.json().await.expect("json body");
    assert!(body.error.is_none());

    let payload = general_purpose::STANDARD
        .decode(body.payload_b64)
        .expect("decode payload");
    let message = String::from_utf8(payload).expect("utf-8 response");
    assert_eq!(message, "2025-12-21T00:00:00Z hello world");

    server_task.abort();
    let _ = server_task.await;
}
