use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    error::AppError,
    models::{CreateTodo, Todo, UpdateTodo},
    state::AppState,
    store::TodoStore,
};

pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let items = TodoStore::list(&state).await?;
    Ok(Json(items))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::BadRequest("title is empty"));
    }

    let todo = TodoStore::create(&state, title.to_string()).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = TodoStore::get(&state, id).await?;
    Ok(Json(todo))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let title = if let Some(t) = payload.title {
        let t = t.trim();
        if t.is_empty() {
            return Err(AppError::BadRequest("title is empty"));
        }
        Some(t.to_string())
    } else {
        None
    };

    let todo = TodoStore::update(&state, id, title, payload.done).await?;

    Ok(Json(todo))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    TodoStore::delete(&state, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
