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

#[server]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use crate::server::db::expect_pool;

    let title = title.trim().to_owned();
    if title.is_empty() {
        return Err(ServerFnError::new("title must not be empty"));
    }

    let pool = expect_pool();
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed, created_at)
         VALUES (?1, FALSE, CAST(unixepoch() AS INTEGER))
         RETURNING id, title, completed, created_at",
    )
    .bind(title)
    .fetch_one(&pool)
    .await?;

    Ok(todo)
}
