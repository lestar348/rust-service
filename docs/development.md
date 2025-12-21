# Development (Local-first)

## Primary mode
- x86_64 Linux (native or VM)
- use MockTransport by default
- optional: run dev http server (devtools) + HttpTransport

## Tooling
- `cargo test` runs all unit + integration tests
- `RUST_LOG`/config controls tracing verbosity
- feature flags:
  - `transport_mock` (default)
  - `transport_http`
  - `transport_ble`

## Debugging rules
- Always include `request_id` in logs for RPC.
- Prefer deterministic tests: fixed clocks via injected clock in future (not MVP).