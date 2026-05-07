# Product Snapshot

## What This Project Is

This repository is the early scaffold for a full-stack Rust TodoMVC-style web app. It is currently a freshly initialized `cargo-leptos` project, not yet a todo application.

## What It Does Today

- Serves a starter Leptos app through an SSR-capable Axum binary.
- Hydrates the same app in the browser through a separate client build.
- Renders a basic home page with the default Leptos counter interaction.
- Builds with `cargo leptos build --release`.

## Current Architecture

- Single Rust package with feature-split builds:
  - `ssr` enables the server binary path.
  - `hydrate` enables the browser/WASM path.
- `src/main.rs` now wires the server through `leptos_axum`:
  - route generation via `generate_route_list`
  - Leptos route mounting via `leptos_routes`
  - fallback handling via `file_and_error_handler(shell)`
- The server listens on the configured `LEPTOS_SITE_ADDR`.
- Leptos metadata is configured in `Cargo.toml`:
  - output name: `alex-hou-2024-test-14`
  - site address: `0.0.0.0:8080`
  - assets directory: `public`
  - style entry: `style/main.scss`

## Conventions and Boundaries

- Treat this as a Leptos full-stack app and use `cargo leptos` for production builds.
- The checked-in code reflects scaffolding and server wiring through issue `#2`.
- Todo domain models, persistence, environment config, Docker, and product-specific UI are not implemented yet.
