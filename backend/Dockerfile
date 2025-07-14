FROM rust:1.87 as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
RUN cargo build --release || true
COPY src ./src
RUN cargo build --release

#Stage 2
FROM debian:bookworm
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/system_metrics /usr/local/bin/system_metrics
EXPOSE 50000

CMD ["system_metrics"]
