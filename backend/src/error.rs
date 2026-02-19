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
    // メールアドレスが既に登録されている
    Conflict,
    // メールアドレスまたはパスワードが間違っている
    Unauthorized,
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
            AppError::Conflict => {
                let body = ErrorBody {
                    error: "既に登録されているメールアドレスです".to_string(),
                };
                (StatusCode::CONFLICT, Json(body)).into_response()
            }
            AppError::Unauthorized => {
                let body = ErrorBody {
                    error: "メールアドレスまたはパスワードが正しくありません".to_string(),
                };
                (StatusCode::UNAUTHORIZED, Json(body)).into_response()
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationError;

    mod from_validation_errors {
        use super::*;

        #[test]
        fn エラーメッセージが正しく変換される() {
            let mut errors = ValidationErrors::new();
            let mut error = ValidationError::new("length");
            error.message = Some("タイトルは1文字以上200文字以下で入力してください".into());
            errors.add("title", error);

            let app_error = AppError::from(errors);

            // AppError::Validation でなければ panic してテスト失敗
            let AppError::Validation(msg) = app_error else {
                panic!("AppError::Validation が期待されたが、別のバリアントだった");
            };
            assert!(msg.contains("title"));
            assert!(msg.contains("タイトルは1文字以上200文字以下で入力してください"));
        }

        #[test]
        fn 複数フィールドのエラーがセミコロンで結合される() {
            let mut errors = ValidationErrors::new();

            let mut error1 = ValidationError::new("length");
            error1.message = Some("タイトルエラー".into());
            errors.add("title", error1);

            let mut error2 = ValidationError::new("length");
            error2.message = Some("説明エラー".into());
            errors.add("description", error2);

            let app_error = AppError::from(errors);

            let AppError::Validation(msg) = app_error else {
                panic!("AppError::Validation が期待された");
            };
            assert!(msg.contains("title: タイトルエラー"));
            assert!(msg.contains("description: 説明エラー"));
            assert!(msg.contains("; "));
        }

        #[test]
        fn メッセージなしのエラーは無視される() {
            let mut errors = ValidationErrors::new();
            let error = ValidationError::new("length");
            errors.add("title", error);

            let app_error = AppError::from(errors);

            let AppError::Validation(msg) = app_error else {
                panic!("AppError::Validation が期待された");
            };
            assert_eq!(msg, "");
        }
    }
}
