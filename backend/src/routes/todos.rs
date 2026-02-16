use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreateTodo, Todo, UpdateTodo},
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

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.lock().await;

    let todo = todos
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.lock().await;

    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        todo.title = title;
    }

    if let Some(done) = payload.done {
        todo.done = done;
    }

    Ok(Json(todo.clone()))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let mut todos = state.todos.lock().await;

    let before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == before {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
