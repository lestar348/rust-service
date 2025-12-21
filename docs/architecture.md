# Architecture

## Goals
- Run on Raspberry Pi 4 and Raspberry Pi Zero.
- Auto-select transport on startup: Wi-Fi => HTTP, no Wi-Fi => BLE.
- Support both RPC (request->response) and events (push).
- Allow manual transport override + observer callback on changes.
- Feature modules are transport-agnostic and initialized via DI.
- Designed for testability (mock transports) and local development (x86_64 Linux).

## Non-Goals (for MVP)
- Strong authentication/authorization.
- Persistent storage layer.
- Remote update mechanism.
- Multi-process separation.

---

## High-Level Components
### app (binary)
- loads config
- initializes logging/tracing
- constructs TransportManager
- constructs FeatureRegistry
- starts runtime and blocks until shutdown signal

### core (contracts)
- `Transport` trait (async): `start/shutdown/call/subscribe`
- `TransportManagerApi`: `init/switch_to/current/on_transport_changed`
- `Feature` trait: `name/init/shutdown`
- shared models: `RpcRequest`, `RpcResponse`, `TransportEvent`, versioning

### transport (implementations)
- `MockTransport`: in-memory router + broadcast events
- `HttpTransport`: RPC over HTTP + events over WS/SSE
- `BleTransport`: GATT RPC + notify events with framing/chunking

### platform (environment)
- `WifiDetector`: determine Wi-Fi availability (real for Linux, mock for tests)

### features
- `HelloWorldFeature`: exposes `hello.get` => "<RFC3339 datetime> hello world"

---

## Concurrency & State Model

### Subscription model
- Events: unified subscription per transport via `Transport::subscribe()`.
- Transport changes: `TransportManagerApi::on_transport_changed()` returns a subscription.

### Switching semantics
- Switch is *atomic* from the consumer viewpoint:
  - new `TransportHandle` published
  - `TransportChanged {old,new}` event emitted
  - old transport gracefully shutdown
- Override priority:
  - if override active: auto-selection is ignored
  - override can be cleared to re-enable auto-selection

### Ownership & DI
- `TransportHandle` carries `Arc<dyn Transport>` (Send+Sync).
- Features receive `FeatureContext { transport: TransportHandle, ... }` during `init`.

---

## Public Contracts

### Transport
- RPC:
  - input: `RpcRequest { service, method, payload, timeout_ms }`
  - output: `RpcResponse { payload }`
- Events:
  - `TransportEvent { topic, payload }`
- Serialization:
  - selected by config (`json` / `cbor`), versioned by `protocol_version`.

### Feature API naming
- Stable naming:
  - `service`: static string (e.g. "hello")
  - `method`: static string (e.g. "get")
- Payload schema:
  - versioned per method if needed (prefer a single global `protocol_version` in MVP)

---

## Testing Strategy
- Unit:
  - core contract tests (serialization boundaries)
  - mock router tests
- Integration (x86_64):
  - dev http server + HttpTransport + feature => hello.get
- On-device smoke tests:
  - at least one RPC call and one event on Pi4/PiZero

---

## Config Model (overview)
- transport:
  - `auto_select: bool`
  - `override: none|http|ble|mock`
  - http:
    - `base_url`, `events: ws|sse`, timeouts
  - ble:
    - uuids, mtu assumptions, timeouts
- logging:
  - `format: pretty|json`, level
- runtime:
  - tokio worker count, shutdown grace

---

## Observability
- tracing spans:
  - `transport.init`
  - `transport.switch`
  - `rpc.call`
  - `event.recv`
- required fields:
  - `transport_kind`, `request_id`, `service`, `method`, `latency_ms`