# Alex-Hou-2024-test-14

Early full-stack Leptos scaffold for a TodoMVC-style app. The current codebase provides the Leptos/Axum starter app, env-file loading, and Docker packaging. Todo-specific features are not implemented yet.

## Prerequisites

- Rust toolchain
- `wasm32-unknown-unknown` target
- `cargo-leptos`

Example setup:

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-leptos
```

## Environment

Copy the example file if you want local overrides:

```bash
cp .env.example .env
```

| Variable | Purpose | Current default |
| --- | --- | --- |
| `DATABASE_URL` | Reserved for upcoming server-side persistence work. | `sqlite://target/app.db` |
| `LEPTOS_SITE_ADDR` | Address the Axum server binds to. | `0.0.0.0:8080` |
| `LEPTOS_OUTPUT_NAME` | Asset bundle name used by cargo-leptos. | `alex-hou-2024-test-14` |

## Local Development

Run the Leptos watcher for the SSR server and hydrate bundle:

```bash
cargo leptos watch
```

The app will bind to `0.0.0.0:8080` by default.

## Release Build

Build the server binary and frontend assets:

```bash
cargo leptos build --release
```

Run the built server locally:

```bash
LEPTOS_SITE_ADDR=0.0.0.0:8080 ./target/release/alex-hou-2024-test-14
```

## Docker

Build the multi-stage image:

```bash
docker build -t alex-hou-2024-test-14 .
```

Run it with the container port and data volume from the current Dockerfile:

```bash
docker run --rm -p 3000:3000 -v "$(pwd)/data:/data" alex-hou-2024-test-14
```

The container runtime sets `LEPTOS_SITE_ADDR=0.0.0.0:3000` and declares `/data` as a volume.
