use axum::{Router, routing::get};

use crate::{routes, state::AppState};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(routes::todos::list_todos))
        .with_state(state)
}
