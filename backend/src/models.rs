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
    #[validate(length(min = 1, max = 200, message = "タイトルは1文字以上200文字以下で入力してください"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 200, message = "タイトルは1文字以上200文字以下で入力してください"))]
    pub title: Option<String>,
    pub done: Option<bool>,
}