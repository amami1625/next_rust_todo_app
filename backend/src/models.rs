use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TodoRow {
    pub id: i64,
    pub title: String,
    pub done: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoGet {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl From<TodoRow> for TodoGet {
    fn from(row: TodoRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            done: row.done != 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub done: Option<bool>,
}
