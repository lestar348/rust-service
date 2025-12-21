# Service Project

A skeleton Rust workspace for a Raspberry Pi service that can expose APIs over HTTP or BLE and includes development tooling for desktop Linux.

## Workspace layout

The project is organized as a Cargo workspace with crates for core abstractions, transports, features, platform integrations, the main app, and development tools. Configuration, scripts, and documentation live at the workspace root.

## Building

```bash
cargo build
```

## Running the app locally (mock transport)

```bash
cargo run -p service-app --features "use_transport_mock"
```

Pass a custom configuration file path as the first argument:

```bash
cargo run -p service-app -- ./configs/default.toml
```

## Development tools

A placeholder `service-devtools` binary is available for future diagnostics:

```bash
cargo run -p service-devtools
```
