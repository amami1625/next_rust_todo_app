use axum::{Router, routing::get};

use crate::{routes, state::AppState};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route(
            "/todos",
            get(routes::todos::list_todos).post(routes::todos::create_todo),
        )
        .route("/todos/{id}", get(routes::todos::get_todo))
        .with_state(state)
}
