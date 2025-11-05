FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-orchestrator /usr/local/bin/

WORKDIR /app
COPY config.yaml .

CMD ["rust-orchestrator"]