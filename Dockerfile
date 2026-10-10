FROM rust:1.85-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release

FROM alpine:3.21
COPY --from=builder /app/target/release/powerset /usr/local/bin/powerset
ENTRYPOINT ["/usr/local/bin/powerset"]
