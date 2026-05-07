# Product Snapshot

## What This Project Is

This repository is the early scaffold for a full-stack Rust TodoMVC-style web app. It is currently a freshly initialized `cargo-leptos` project, not yet a todo application.

## What It Does Today

- Serves a starter Leptos app through an SSR-capable Axum binary.
- Hydrates the same app in the browser through a separate client build.
- Renders a basic home page with the default Leptos counter interaction.
- Loads runtime environment variables from `.env` on server startup when present.
- Builds with `cargo leptos build --release`.
- Ships with a multi-stage Docker build for container packaging.
- Includes a README with local dev, release build, Docker run, and env-var instructions.

## Current Architecture

- Single Rust package with feature-split builds:
  - `ssr` enables the server binary path.
  - `hydrate` enables the browser/WASM path.
- `src/main.rs` now wires the server through `leptos_axum`:
  - route generation via `generate_route_list`
  - Leptos route mounting via `leptos_routes`
  - fallback handling via `file_and_error_handler(shell)`
- The server listens on the configured `LEPTOS_SITE_ADDR`.
- Server startup loads env files through `dotenvy` before Leptos configuration is read.
- Leptos metadata is configured in `Cargo.toml`:
  - output name: `alex-hou-2024-test-14`
  - site address: `0.0.0.0:8080`
  - assets directory: `public`
  - style entry: `style/main.scss`
- Shared models live in `src/models.rs` and currently include:
  - `Todo { id, title, completed, created_at }`
  - `Filter { All, Active, Completed }`
- `sqlx` is now wired into the `ssr` feature set for upcoming server-side SQLite access.
- The repository includes the first SQLite migration:
  - `migrations/0001_create_todos.sql`
  - schema columns: `id`, `title`, `completed`, `created_at`
- Server-side DB bootstrap now lives in `src/server/db.rs`:
  - opens `DATABASE_URL`
  - ensures the SQLite file path exists
  - runs `sqlx::migrate!()` at startup
  - exposes the pooled connection through shared server context/state
- Shared todo server functions now live in `src/todos.rs`:
  - `list_todos` returns `Vec<Todo>`
  - results are ordered by `created_at` with `id` as a stable tiebreaker
  - intended for SSR preload and client refetch paths
  - `add_todo` trims input, rejects blank titles, and returns the created row
  - `update_todo` supports title edits and completion toggles
  - empty trimmed edit titles delete the row and return `None`
  - `delete_todo` removes one row and errors on missing IDs
  - `clear_completed` removes all completed rows and returns a deleted-row count
- `.env.example` documents:
  - `DATABASE_URL`
  - `LEPTOS_SITE_ADDR`
  - `LEPTOS_OUTPUT_NAME`
- Container packaging is defined by:
  - a multi-stage `Dockerfile`
  - a slim runtime image with the server binary, `target/site`, and `migrations/`
  - container port `3000` and a `/data` volume

## Conventions and Boundaries

- Treat this as a Leptos full-stack app and use `cargo leptos` for production builds.
- The checked-in repo includes operational docs for `cargo leptos watch`, release builds, Docker usage, and env vars.
- The checked-in code reflects scaffolding, server wiring, env-loading support, Docker packaging, README guidance, shared todo/filter models, SQLite schema work, DB pool initialization, and the initial todo read/create/update/delete server-function layer through issue `#12`.
- Product-specific UI and the remaining toggle-all mutation flow are not implemented yet.
