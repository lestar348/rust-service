# Deployment

## Targets
- Raspberry Pi 4: aarch64-unknown-linux-gnu
- Raspberry Pi Zero: armv6-unknown-linux-gnueabihf

## Artifacts
- `service_app` binary
- `configs/*.toml`
- `systemd/service-app.service`

## systemd
- restart=on-failure
- env/config path via EnvironmentFile or args

## Smoke checklist
- service starts
- logs appear
- `hello.get` works (via chosen transport)
- one event received (if enabled)