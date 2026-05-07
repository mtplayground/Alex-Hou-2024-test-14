use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::{
    fs::{self, File},
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
}

impl AppState {
    pub fn new(leptos_options: LeptosOptions, pool: SqlitePool) -> Self {
        Self {
            leptos_options,
            pool,
        }
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

pub async fn init_pool() -> Result<SqlitePool, DbError> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::new(ErrorKind::NotFound, "DATABASE_URL must be set"))?;

    ensure_sqlite_file_exists(&database_url)?;

    let connect_options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub fn expect_pool() -> SqlitePool {
    leptos::prelude::expect_context::<SqlitePool>()
}

fn ensure_sqlite_file_exists(database_url: &str) -> Result<(), DbError> {
    let Some(path) = sqlite_file_path(database_url) else {
        return Ok(());
    };

    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        File::options()
            .create(true)
            .write(true)
            .truncate(false)
            .open(path)?;
    }

    Ok(())
}

fn sqlite_file_path(database_url: &str) -> Option<PathBuf> {
    if !database_url.starts_with("sqlite:") {
        return None;
    }

    let path = database_url.strip_prefix("sqlite:")?;
    let path = path.split_once('?').map_or(path, |(path, _)| path);
    let path = path.strip_prefix("//").unwrap_or(path);

    if path.is_empty() || path == ":memory:" {
        return None;
    }

    Some(PathBuf::from(path))
}
