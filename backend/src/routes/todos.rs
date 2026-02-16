use axum::{Json, extract::State, http::StatusCode};

use crate::{
    models::{CreateTodo, Todo},
    state::AppState,
};

pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let snapshot = {
        let todos = state.todos.lock().await;
        todos.clone()
    };

    Json(snapshot)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.todos.lock().await;

    let id = (todos.len() as u32) + 1;

    let todo = Todo {
        id,
        title: payload.title,
        done: false,
    };

    todos.push(todo.clone());

    (StatusCode::CREATED, Json(todo))
}
