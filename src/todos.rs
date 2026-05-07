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

#[server]
pub async fn update_todo(
    id: i64,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Option<Todo>, ServerFnError> {
    use crate::server::db::expect_pool;

    let pool = expect_pool();
    let title = title.map(|value| value.trim().to_owned());

    if matches!(title.as_deref(), Some("")) {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ServerFnError::new("todo not found"));
        }

        return Ok(None);
    }

    let current = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed, created_at FROM todos WHERE id = ?1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ServerFnError::new("todo not found"))?;

    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos
         SET title = ?2, completed = ?3
         WHERE id = ?1
         RETURNING id, title, completed, created_at",
    )
    .bind(id)
    .bind(title.unwrap_or(current.title))
    .bind(completed.unwrap_or(current.completed))
    .fetch_one(&pool)
    .await?;

    Ok(Some(todo))
}

#[server]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use crate::server::db::expect_pool;

    let pool = expect_pool();
    let result = sqlx::query("DELETE FROM todos WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new("todo not found"));
    }

    Ok(())
}

#[server]
pub async fn clear_completed() -> Result<u64, ServerFnError> {
    use crate::server::db::expect_pool;

    let pool = expect_pool();
    let result = sqlx::query("DELETE FROM todos WHERE completed = TRUE")
        .execute(&pool)
        .await?;

    Ok(result.rows_affected())
}

#[server]
pub async fn toggle_all(completed: bool) -> Result<u64, ServerFnError> {
    use crate::server::db::expect_pool;

    let pool = expect_pool();
    let result = sqlx::query("UPDATE todos SET completed = ?1")
        .bind(completed)
        .execute(&pool)
        .await?;

    Ok(result.rows_affected())
}
