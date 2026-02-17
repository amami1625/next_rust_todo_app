use crate::{error::AppError, models::Todo, state::AppState};

pub struct TodoStore;

impl TodoStore {
    pub async fn list(state: &AppState) -> Vec<Todo> {
        let todos = state.todos.lock().await;
        todos.clone()
    }

    pub async fn get(state: &AppState, id: u32) -> Result<Todo, AppError> {
        let todos = state.todos.lock().await;
        todos
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or(AppError::NotFound)
    }

    pub async fn create(state: &AppState, title: String) -> Todo {
        let mut todos = state.todos.lock().await;
        let id = state.allocate_id();

        let todo = Todo {
            id,
            title,
            done: false,
        };

        todos.push(todo.clone());
        todo
    }

    pub async fn update(
        state: &AppState,
        id: u32,
        title: Option<String>,
        done: Option<bool>,
    ) -> Result<Todo, AppError> {
        let mut todos = state.todos.lock().await;

        let todo = todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(AppError::NotFound)?;

        if let Some(title) = title {
            todo.title = title;
        }

        if let Some(done) = done {
            todo.done = done;
        }

        Ok(todo.clone())
    }

    pub async fn delete(state: &AppState, id: u32) -> Result<(), AppError> {
        let mut todos = state.todos.lock().await;
        let before = todos.len();
        todos.retain(|t| t.id != id);

        if todos.len() == before {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
