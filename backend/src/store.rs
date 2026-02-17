use crate::{
    error::AppError,
    models::{TodoGet, TodoRow},
    state::AppState,
};

pub struct TodoStore;

impl TodoStore {
    pub async fn list(state: &AppState) -> Result<Vec<TodoGet>, AppError> {
        let rows = sqlx::query_as::<_, TodoRow>(
            "SELECT id, title, created_at, updated_at, done FROM todos ORDER BY id",
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| AppError::Internal)?;

        Ok(rows.into_iter().map(TodoGet::from).collect())
    }

    pub async fn get(state: &AppState, id: i64) -> Result<TodoGet, AppError> {
        let row = sqlx::query_as::<_, TodoRow>(
            "SELECT id, title, done, created_at, updated_at FROM todos WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| AppError::Internal)?;

        let row = row.ok_or(AppError::NotFound)?;
        Ok(TodoGet::from(row))
    }

    pub async fn create(state: &AppState, title: String) -> Result<TodoGet, AppError> {
        let res = sqlx::query(
            r#"
            WITH now(ts) AS (SELECT datetime('now'))
            INSERT INTO todos (title, done, created_at, updated_at)
            SELECT ?, 0, ts, ts FROM now
            "#
        )
        .bind(title)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][create] {e:?}");
            AppError::Internal
        })?;

        let id = res.last_insert_rowid();
        Self::get(state, id).await
    }

    pub async fn update(
        state: &AppState,
        id: i64,
        title: Option<String>,
        done: Option<bool>,
    ) -> Result<TodoGet, AppError> {
        let _ = Self::get(state, id).await?;

        if let Some(title) = title {
            sqlx::query("UPDATE todos SET title = ?, updated_at = datetime('now') WHERE id = ?")
                .bind(title)
                .bind(id)
                .execute(&state.pool)
                .await
                .map_err(|_| AppError::Internal)?;
        }

        if let Some(done) = done {
            let done_1 = if done { 1 } else { 0 };
            sqlx::query("UPDATE todos SET done = ?, updated_at = datetime('now') WHERE id = ?")
                .bind(done_1)
                .bind(id)
                .execute(&state.pool)
                .await
                .map_err(|_| AppError::Internal)?;
        }

        Self::get(state, id).await
    }

    pub async fn delete(state: &AppState, id: i64) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&state.pool)
            .await
            .map_err(|_| AppError::Internal)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
