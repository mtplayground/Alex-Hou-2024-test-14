#[cfg(feature = "ssr")]
type AppError = Box<dyn std::error::Error + Send + Sync>;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    use alex_hou_2024_test_14::server::db::{self, AppState};
    use leptos::prelude::*;

    load_environment()?;
    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let pool = db::init_pool().await?;
    let state = AppState::new(leptos_options, pool);
    let addr = state.leptos_options.site_addr;
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
fn load_environment() -> Result<(), AppError> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

#[cfg(feature = "ssr")]
fn build_router(state: alex_hou_2024_test_14::server::db::AppState) -> axum::Router {
    use alex_hou_2024_test_14::app::{shell, App};
    use alex_hou_2024_test_14::server::db::AppState;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::provide_context;
    use leptos_axum::{file_and_error_handler_with_context, generate_route_list, LeptosRoutes};

    let routes = generate_route_list(App);
    let shell_options = state.leptos_options.clone();
    let context_state = state.clone();
    let additional_context = move || {
        provide_context::<AppState>(context_state.clone());
        provide_context(context_state.pool.clone());
    };

    log!(
        "listening on http://{} (resolved from LEPTOS_SITE_ADDR)",
        state.leptos_options.site_addr
    );

    Router::new()
        .leptos_routes_with_context(&state, routes, additional_context.clone(), move || {
            shell(shell_options.clone())
        })
        .fallback(file_and_error_handler_with_context::<AppState, _>(
            additional_context,
            shell,
        ))
        .with_state(state)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
