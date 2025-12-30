use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::todos;

/// API ルーターを作成
///
/// すべての API エンドポイントを定義して返す
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        // /api/todos のルート
        .route(
            "/api/todos",
            get(todos::get_todos) // GET /api/todos → 全件取得
                .post(todos::create_todo), // POST /api/todos → 新規作成
        )
        // /api/todos/:id のルート
        .route(
            "/api/todos/:id",
            get(todos::get_todo) // GET /api/todos/:id → 1件取得
                .put(todos::update_todo) // PUT /api/todos/:id → 更新
                .delete(todos::delete_todo), // DELETE /api/todos/:id → 削除
        )
        // State として pool を登録（全ハンドラーで共有）
        .with_state(pool)
}
