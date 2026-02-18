use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Validation(String),
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

// From トレイトを実装すると ? 演算子で ValidationErrors → AppError に自動変換される
impl From<ValidationErrors> for AppError {
    fn from(e: ValidationErrors) -> Self {
        let mut messages = Vec::new();

        // field_errors() は HashMap<フィールド名, Vec<エラー情報>> を返す
        // 例: {"title": [ValidationError { message: Some("タイトルは..."), ... }]}
        for (field, errors) in e.field_errors() {
            for error in errors {
                if let Some(msg) = &error.message {
                    messages.push(format!("{}: {}", field, msg));
                }
            }
        }

        AppError::Validation(messages.join("; "))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                let body = ErrorBody {
                    error: "見つかりませんでした".to_string(),
                };
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
            AppError::Validation(msg) => {
                let body = ErrorBody { error: msg };
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            AppError::Internal => {
                let body = ErrorBody {
                    error: "内部エラーが発生しました".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
