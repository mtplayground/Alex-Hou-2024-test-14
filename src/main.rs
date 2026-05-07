#[cfg(feature = "ssr")]
type AppError = Box<dyn std::error::Error + Send + Sync>;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    use leptos::prelude::*;

    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let app = build_router(leptos_options);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
fn build_router(leptos_options: leptos::config::LeptosOptions) -> axum::Router {
    use alex_hou_2024_test_14::app::{shell, App};
    use axum::Router;
    use leptos::logging::log;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let routes = generate_route_list(App);
    let shell_options = leptos_options.clone();

    log!(
        "listening on http://{} (resolved from LEPTOS_SITE_ADDR)",
        leptos_options.site_addr
    );

    Router::new()
        .leptos_routes(&leptos_options, routes, move || {
            shell(shell_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
