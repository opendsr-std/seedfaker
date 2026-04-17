FROM rust:1-slim AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY rust/ rust/
RUN cargo build --release --manifest-path rust/Cargo.toml -p seedfaker \
    && cp rust/target/release/seedfaker /usr/local/bin/seedfaker

FROM debian:bookworm-slim
COPY --from=builder /usr/local/bin/seedfaker /usr/local/bin/seedfaker
ENTRYPOINT ["seedfaker"]
