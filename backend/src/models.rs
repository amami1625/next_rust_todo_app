use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(length(
        min = 1,
        max = 200,
        message = "タイトルは1文字以上200文字以下で入力してください"
    ))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(
        min = 1,
        max = 200,
        message = "タイトルは1文字以上200文字以下で入力してください"
    ))]
    pub title: Option<String>,
    pub done: Option<bool>,
}

// クエリパラメータを受け取る struct
// GET /todos?page=1&limit=10 のように使う
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// ページネーション付きのレスポンス
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_todo {
        use super::*;

        #[test]
        fn 正常なタイトルでバリデーションが通る() {
            let todo = CreateTodo {
                title: "買い物に行く".to_string(),
            };
            assert!(todo.validate().is_ok());
        }

        #[test]
        fn 空のタイトルはバリデーションエラーになる() {
            let todo = CreateTodo {
                title: "".to_string(),
            };
            assert!(todo.validate().is_err());
        }

        #[test]
        fn タイトル200文字はバリデーションが通る() {
            let todo = CreateTodo {
                title: "あ".repeat(200),
            };
            assert!(todo.validate().is_ok());
        }

        #[test]
        fn タイトル201文字はバリデーションエラーになる() {
            let todo = CreateTodo {
                title: "あ".repeat(201),
            };
            assert!(todo.validate().is_err());
        }
    }

    mod update_todo {
        use super::*;

        #[test]
        fn タイトルなしの更新はバリデーションが通る() {
            let todo = UpdateTodo {
                title: None,
                done: Some(true),
            };
            assert!(todo.validate().is_ok());
        }

        #[test]
        fn 正常なタイトルでの更新はバリデーションが通る() {
            let todo = UpdateTodo {
                title: Some("新しいタイトル".to_string()),
                done: None,
            };
            assert!(todo.validate().is_ok());
        }

        #[test]
        fn タイトル200文字はバリデーションが通る() {
            let todo = UpdateTodo {
                title: Some("a".repeat(200)),
                done: Some(true),
            };
            assert!(todo.validate().is_ok());
        }

        #[test]
        fn タイトル201文字はバリデーションエラーになる() {
            let todo = UpdateTodo {
                title: Some("あ".repeat(201)),
                done: Some(true),
            };
            assert!(todo.validate().is_err());
        }

        #[test]
        fn 空のタイトルでの更新はバリデーションエラーになる() {
            let todo = UpdateTodo {
                title: Some("".to_string()),
                done: None,
            };
            assert!(todo.validate().is_err());
        }
    }
}
