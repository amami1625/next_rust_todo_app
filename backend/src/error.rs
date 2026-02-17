use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(&'static str),
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                let body = ErrorBody { error: "not found" };
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
            AppError::BadRequest(msg) => {
                let body = ErrorBody { error: msg };
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
        }
    }
}
