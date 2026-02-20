use axum::{Router, http::HeaderValue, middleware::from_fn_with_state, routing::{get, post}};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

use crate::{
    handler::{
        auth::{login, register},
        middleware::require_auth,
        todos::{create_todo, delete_todo, get_todo, list_todos, update_todo},
    },
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    // CORS 設定
    // ブラウザは異なるオリジン（ポート違いも含む）へのリクエストをデフォルトで拒否する
    // サーバー側で「このオリジンからのリクエストは許可する」と明示する必要がある
    let cors = CorsLayer::new()
        // フロントエンドの URL を許可（localhost:3001）
        .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        // 使用する HTTP メソッドを許可
        .allow_methods(AllowMethods::any())
        // Authorization ヘッダーなどを許可
        .allow_headers(AllowHeaders::any());

    // 認証が必要な Todo ルート
    let protected = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route_layer(from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .merge(protected)
        .layer(cors)
        .with_state(state)
}
