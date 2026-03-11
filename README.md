# Khamoshchat Broker

A custom [RMQTT](https://github.com/rmqtt/rmqtt) broker with an integrated plugin that logs MQTT client lifecycle events for observability.

## What It Does

The **Khamoshchat Plugin** hooks into the following MQTT events and logs them:

| Event | Logged Info |
|---|---|
| `ClientConnect` | Client IP address |
| `ClientAuthenticate` | Client ID, username |
| `ClientSubscribeCheckAcl` | Client ID, topic filter |
| `SessionSubscribed` | Client ID, subscribed topic |
| `MessagePublish` | Topic, sender, payload |

## Project Structure

```
khamoshchat-broker/
├── Cargo.toml          # Dependencies & package metadata
├── config/
│   └── default.toml    # Default broker configuration
├── src/
│   ├── main.rs         # Broker entrypoint (TCP listener, plugin registration)
│   └── plugin.rs       # KhamoshchatPlugin (event hooks & logging)
├── Dockerfile
└── README.md
```

## Quick Start

### Build

```bash
cargo build --release
```

### Run

```bash
# Default: listens on 0.0.0.0:1883
cargo run --release

# Custom host/port via environment variables
BROKER_HOST=127.0.0.1 BROKER_PORT=1884 cargo run --release
```

### Test with an MQTT Client

```bash
# Subscribe (in one terminal)
mosquitto_sub -h localhost -t "test/topic"

# Publish (in another terminal)
mosquitto_pub -h localhost -t "test/topic" -m "Hello, Khamoshchat!"
```

You'll see structured logs in the broker console:
```
[INFO khamoshchat] Client connecting from 127.0.0.1:56789
[INFO khamoshchat] Client authenticate - client_id: auto-xxx, username: ...
[INFO khamoshchat] Message published on topic 'test/topic' from '...': Hello, Khamoshchat!
```

### Run Tests

```bash
cargo test
```

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `BROKER_HOST` | `0.0.0.0` | TCP bind address |
| `BROKER_PORT` | `1883` | TCP listen port |
| `RUST_LOG` | `info` | Log level filter (e.g. `khamoshchat=debug`) |

## Docker

```bash
docker build -t khamoshchat-broker .
docker run -p 1883:1883 khamoshchat-broker
```
