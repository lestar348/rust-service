use std::env;

fn main() {
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/etc/service-project/config.toml".to_string());

    println!(
        "Starting service-app with config: {} (transport features selected via Cargo)",
        config_path
    );
}
