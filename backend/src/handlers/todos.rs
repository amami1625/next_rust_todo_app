use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateTodo, Todo, UpdateTodo};

/// Todo を新規作成
///
/// POST /api/todos
/// Body: {"title": "タイトル", "description": "説明（任意）"}
pub async fn create_todo(
    State(pool): State<PgPool>,           // データベース接続プールを取得
    Json(payload): Json<CreateTodo>,      // リクエストボディを CreateTodo に変換
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    // SQL クエリで新しい Todo を挿入し、作成されたレコードを返す
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, completed, created_at, updated_at
        "#,
    )
    .bind(&payload.title)           // $1 にタイトルをバインド
    .bind(&payload.description)     // $2 に説明をバインド
    .fetch_one(&pool)               // クエリを実行して1件取得
    .await
    .map_err(|e| {
        // エラーが発生した場合
        tracing::error!("Failed to create todo: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create todo: {}", e))
    })?;

    // 201 Created ステータスコードと作成された Todo を返す
    Ok((StatusCode::CREATED, Json(todo)))
}

/// 全ての Todo を取得
///
/// GET /api/todos
pub async fn get_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    // 全ての Todo を作成日時の降順で取得
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, description, completed, created_at, updated_at
        FROM todos
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)               // 全件取得
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch todos: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch todos: {}", e))
    })?;

    Ok(Json(todos))
}

/// 特定の Todo を取得
///
/// GET /api/todos/:id
pub async fn get_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,              // URL パスから ID を取得
) -> Result<Json<Todo>, (StatusCode, String)> {
    // 指定された ID の Todo を取得
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, description, completed, created_at, updated_at
        FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)          // 存在しない場合は None を返す
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch todo: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch todo: {}", e))
    })?
    .ok_or_else(|| {
        // Todo が見つからない場合は 404 を返す
        (StatusCode::NOT_FOUND, format!("Todo with id {} not found", id))
    })?;

    Ok(Json(todo))
}

/// Todo を更新
///
/// PUT /api/todos/:id
/// Body: {"title": "新タイトル", "completed": true} など
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    // まず対象の Todo を取得
    let mut todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch todo: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch todo: {}", e))
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Todo with id {} not found", id)))?;

    // 更新するフィールドを適用
    if let Some(title) = payload.title {
        todo.title = title;
    }
    // description の処理: None = 更新しない, Some(None) = null に設定, Some(Some(v)) = 値を設定
    if let Some(description) = payload.description {
        todo.description = description;
    }
    if let Some(completed) = payload.completed {
        todo.completed = completed;
    }

    // データベースを更新
    let updated_todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET title = $1, description = $2, completed = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, description, completed, created_at, updated_at
        "#,
    )
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(todo.completed)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update todo: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update todo: {}", e))
    })?;

    Ok(Json(updated_todo))
}

/// Todo を削除
///
/// DELETE /api/todos/:id
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // 削除を実行
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete todo: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete todo: {}", e))
        })?;

    // 削除された行数を確認
    if result.rows_affected() == 0 {
        // 削除対象が見つからなかった
        return Err((StatusCode::NOT_FOUND, format!("Todo with id {} not found", id)));
    }

    // 204 No Content（削除成功、レスポンスボディなし）
    Ok(StatusCode::NO_CONTENT)
}
