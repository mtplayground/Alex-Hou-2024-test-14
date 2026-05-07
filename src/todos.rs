use crate::models::Todo;
use leptos::prelude::ServerFnError;
use leptos::server;

#[server]
pub async fn list_todos() -> Result<Vec<Todo>, ServerFnError> {
    use crate::server::db::expect_pool;

    let pool = expect_pool();
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed, created_at FROM todos ORDER BY created_at ASC, id ASC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(todos)
}
