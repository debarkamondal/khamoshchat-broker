# --- Build stage ---
FROM rust:1.85-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/khamoshchat-broker /usr/local/bin/khamoshchat-broker
COPY config/ /etc/khamoshchat/

EXPOSE 1883

ENV BROKER_HOST=0.0.0.0
ENV BROKER_PORT=1883
ENV RUST_LOG=info

CMD ["khamoshchat-broker"]
