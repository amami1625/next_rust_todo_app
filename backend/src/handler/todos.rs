use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    error::AppError,
    models::todo::{CreateTodo, PaginatedResponse, Pagination, Todo, UpdateTodo},
    state::AppState,
    store::TodoStore,
};

pub async fn list_todos(
    State(state): State<AppState>,
    // ミドルウェアが req.extensions_mut().insert(user_id) で入れた値を受け取る
    Extension(user_id): Extension<i64>,
    Query(params): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Todo>>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    let (items, total) = TodoStore::list(&state, user_id, page, limit).await?;

    Ok(Json(PaginatedResponse {
        data: items,
        total,
        page,
        limit,
    }))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    payload.validate()?;

    let title = payload.title.trim().to_string();
    let priority = payload.priority.trim().to_string();
    let todo = TodoStore::create(&state, user_id, title, priority).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = TodoStore::get(&state, id, user_id).await?;
    Ok(Json(todo))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    payload.validate()?;

    let title = payload.title.map(|t| t.trim().to_string());
    let priority = payload.priority.map(|p| p.trim().to_string());
    let todo = TodoStore::update(&state, id, user_id, title, priority, payload.done).await?;

    Ok(Json(todo))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    TodoStore::delete(&state, id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
