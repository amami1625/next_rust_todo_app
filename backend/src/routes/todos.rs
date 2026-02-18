use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    error::AppError,
    models::{CreateTodo, PaginatedResponse, Pagination, Todo, UpdateTodo},
    state::AppState,
    store::TodoStore,
};

pub async fn list_todos(
    State(state): State<AppState>,
    Query(params): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Todo>>, AppError> {
    // パラメータが指定されなければデフォルト値を使う
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    let (items, total) = TodoStore::list(&state, page, limit).await?;

    Ok(Json(PaginatedResponse {
        data: items,
        total,
        page,
        limit,
    }))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    payload.validate()?;

    let title = payload.title.trim();
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
    payload.validate()?;

    let title = payload.title.map(|t| t.trim().to_string());
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
