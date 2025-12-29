use std::{env, net::SocketAddr, sync::Arc};

use service_core::{feature::FeatureContext, Feature, SystemClock};
use service_features::hello_world::HelloWorldFeature;
use service_transport::mock::MockTransport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/etc/service-project/config.toml".to_string());

    println!(
        "Starting service-app with config: {} (transport features selected via Cargo)",
        config_path
    );

    let transport = MockTransport::new();
    let registry = transport.registry();
    let events = transport.events();
    let clock = Arc::new(SystemClock);

    let feature = HelloWorldFeature::with_clock(clock.clone());
    feature
        .init(FeatureContext::new(registry.clone(), events, clock))
        .await
        .expect("feature init");

    #[cfg(feature = "use_transport_http")]
    {
        use service_transport::http::HttpServerTransport;

        let addr: SocketAddr = env::var("SERVICE_HTTP_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
            .parse()
            .map_err(|err| anyhow::anyhow!("invalid SERVICE_HTTP_ADDR: {err}"))?;

        println!("Starting HTTP RPC server on {addr}");
        let server = HttpServerTransport::new(registry);
        server.serve(addr).await?;
    }

    #[cfg(not(feature = "use_transport_http"))]
    {
        println!("HTTP transport feature disabled; no server started.");
    }

    Ok(())
}
