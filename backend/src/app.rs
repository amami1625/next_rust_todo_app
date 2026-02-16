use axum::{Router, routing::get};

use crate::{
    routes::todos::{create_todo, delete_todo, get_todo, list_todos, update_todo},
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(state)
}
