FROM rust:1.90-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown \
    && cargo install --locked cargo-leptos

WORKDIR /app

COPY . .

# Issue #7 adds real migrations; keep the runtime layout stable now.
RUN mkdir -p /app/migrations

RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --home-dir /app --shell /usr/sbin/nologin appuser

WORKDIR /app

COPY --from=builder /app/target/release/alex-hou-2024-test-14 /app/alex-hou-2024-test-14
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/migrations /app/migrations

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
VOLUME ["/data"]

USER appuser

ENTRYPOINT ["/app/alex-hou-2024-test-14"]
