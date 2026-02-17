use crate::{error::AppError, models::Todo, state::AppState};

pub struct TodoStore;

impl TodoStore {
    pub async fn list(state: &AppState) -> Result<Vec<Todo>, AppError> {
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, created_at::text AS created_at, updated_at::text AS updated_at, done FROM todos ORDER BY id",
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][list] {e:?}");
            AppError::Internal
        })?;

        Ok(todos)
    }

    pub async fn get(state: &AppState, id: i64) -> Result<Todo, AppError> {
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, title, done, created_at::text AS created_at, updated_at::text AS updated_at FROM todos WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][get] {e:?}");
            AppError::Internal
        })?;

        todo.ok_or(AppError::NotFound)
    }

    pub async fn create(state: &AppState, title: String) -> Result<Todo, AppError> {
        let row = sqlx::query_scalar::<_, i64>(
            r#"
            WITH now(ts) AS (SELECT now())
            INSERT INTO todos (title, done, created_at, updated_at)
            SELECT $1, false, ts, ts FROM now
            RETURNING id 
            "#,
        )
        .bind(title)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][create] {e:?}");
            AppError::Internal
        })?;

        let id = row;
        Self::get(state, id).await
    }

    pub async fn update(
        state: &AppState,
        id: i64,
        title: Option<String>,
        done: Option<bool>,
    ) -> Result<Todo, AppError> {
        let _ = Self::get(state, id).await?;

        if let Some(title) = title {
            sqlx::query("UPDATE todos SET title = $1, updated_at = now() WHERE id = $2")
                .bind(title)
                .bind(id)
                .execute(&state.pool)
                .await
                .map_err(|e| {
                    eprintln!("[sqlx error][update] {e:?}");
                    AppError::Internal
                })?;
        }

        if let Some(done) = done {
            sqlx::query("UPDATE todos SET done = $1, updated_at = now() WHERE id = $2")
                .bind(done)
                .bind(id)
                .execute(&state.pool)
                .await
                .map_err(|e| {
                    eprintln!("[sqlx error][update] {e:?}");
                    AppError::Internal
                })?;
        }

        Self::get(state, id).await
    }

    pub async fn delete(state: &AppState, id: i64) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("[sqlx error][delete] {e:?}");
                AppError::Internal
            })?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
