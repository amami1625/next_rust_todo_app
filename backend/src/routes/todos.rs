use axum::{Json, extract::State};

use crate::{models::Todo, state::AppState};

pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let snapshot = {
        let todos = state.todos.lock().await;
        todos.clone()
    };

    Json(snapshot)
}
