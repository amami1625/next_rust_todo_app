use crate::{error::AppError, models::todo::Todo, state::AppState};

pub struct TodoStore;

impl TodoStore {
    // 戻り値は (Todoリスト, 全件数) のタプル
    // user_id でフィルタリングして、そのユーザーの Todo だけを返す
    pub async fn list(
        state: &AppState,
        user_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<Todo>, i64), AppError> {
        // そのユーザーの全件数を取得
        let total: i64 =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM todos WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&state.pool)
                .await
                .map_err(|e| {
                    eprintln!("[sqlx error][list] {e:?}");
                    AppError::Internal
                })?;

        let offset = (page - 1) * limit;

        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, user_id, title, done, created_at::text AS created_at, updated_at::text AS updated_at FROM todos WHERE user_id = $1 ORDER BY id LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][list] {e:?}");
            AppError::Internal
        })?;

        Ok((todos, total))
    }

    pub async fn get(state: &AppState, id: i64, user_id: i64) -> Result<Todo, AppError> {
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, user_id, title, done, created_at::text AS created_at, updated_at::text AS updated_at FROM todos WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][get] {e:?}");
            AppError::Internal
        })?;

        // user_id が一致しない場合も NotFound を返す（他人の Todo を「見つかりません」で隠す）
        todo.ok_or(AppError::NotFound)
    }

    pub async fn create(state: &AppState, user_id: i64, title: String) -> Result<Todo, AppError> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"
            WITH now(ts) AS (SELECT now())
            INSERT INTO todos (user_id, title, done, created_at, updated_at)
            SELECT $1, $2, false, ts, ts FROM now
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(title)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("[sqlx error][create] {e:?}");
            AppError::Internal
        })?;

        Self::get(state, id, user_id).await
    }

    pub async fn update(
        state: &AppState,
        id: i64,
        user_id: i64,
        title: Option<String>,
        done: Option<bool>,
    ) -> Result<Todo, AppError> {
        // 存在確認（他人の Todo なら NotFound になる）
        let _ = Self::get(state, id, user_id).await?;

        if let Some(title) = title {
            sqlx::query(
                "UPDATE todos SET title = $1, updated_at = now() WHERE id = $2 AND user_id = $3",
            )
            .bind(title)
            .bind(id)
            .bind(user_id)
            .execute(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("[sqlx error][update title] {e:?}");
                AppError::Internal
            })?;
        }

        if let Some(done) = done {
            sqlx::query(
                "UPDATE todos SET done = $1, updated_at = now() WHERE id = $2 AND user_id = $3",
            )
            .bind(done)
            .bind(id)
            .bind(user_id)
            .execute(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("[sqlx error][update done] {e:?}");
                AppError::Internal
            })?;
        }

        Self::get(state, id, user_id).await
    }

    pub async fn delete(state: &AppState, id: i64, user_id: i64) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM todos WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
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
