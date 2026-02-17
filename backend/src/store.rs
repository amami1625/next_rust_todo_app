use crate::{error::AppError, models::Todo, state::AppState};

pub struct TodoStore;

impl TodoStore {
    pub async fn list(state: &AppState) -> Result<Vec<Todo>, AppError> {
        let items = sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos ORDER BY id")
            .fetch_all(&state.pool)
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(items)
    }

    pub async fn get(state: &AppState, id: i64) -> Result<Todo, AppError> {
        let item = sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|_| AppError::Internal)?;

        item.ok_or(AppError::NotFound)
    }

    pub async fn create(state: &AppState, title: String) -> Result<Todo, AppError> {
        let res = sqlx::query("INSERT INTO todos (title, done) VALUES (?, 0)")
            .bind(title)
            .execute(&state.pool)
            .await
            .map_err(|_| AppError::Internal)?;

        let id = res.last_insert_rowid();
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
            sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
                .bind(title)
                .bind(id)
                .execute(&state.pool)
                .await
                .map_err(|_| AppError::Internal)?;
        }

        if let Some(done) = done {
            let done_1 = if done { 1 } else { 0 };
            sqlx::query("UPDATE todos SET done = ? WHERE id = ?")
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
