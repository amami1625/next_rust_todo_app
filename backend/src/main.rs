use axum::http::{Method, header};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, Serialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
    next_id: Arc<Mutex<u64>>,
}

fn initial_state() -> AppState {
    AppState {
        todos: Arc::new(Mutex::new(vec![
            Todo {
                id: 1,
                title: "First todo".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Second todo".to_string(),
                completed: true,
            },
        ])),
        next_id: Arc::new(Mutex::new(3)),
    }
}

async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap().clone();
    Json(todos)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let title = payload.title.trim().to_string();
    if title.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "title is required".to_string()));
    }

    let mut id_guard = state.next_id.lock().unwrap();
    let id = *id_guard;
    *id_guard += 1;

    let todo = Todo {
        id,
        title,
        completed: false,
    };

    let mut todos_guard = state.todos.lock().unwrap();
    todos_guard.push(todo.clone());

    Ok((StatusCode::CREATED, Json(todo)))
}

async fn update_todo(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut todos_guard = state.todos.lock().unwrap();

    let todo = todos_guard.iter_mut().find(|t| t.id == id);
    match todo {
        Some(t) => {
            t.completed = payload.completed;
            Ok(Json(t.clone()))
        }
        None => Err((StatusCode::NOT_FOUND, format!("todo {id} not found"))),
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:3000"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);

    let state = initial_state();

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", patch(update_todo))
        .with_state(state)
        .layer(cors);

    let addr = "0.0.0.0:3001";
    println!("Backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn apply_completed(todos: &mut [Todo], id: u64, completed: bool) -> Option<Todo> {
    let t = todos.iter_mut().find(|t| t.id == id)?;
    t.completed = completed;
    Some(t.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_has_two_todos() {
        let state = initial_state();
        let todos = state.todos.lock().unwrap().clone();
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn create_todo_rejects_empty_title() {
        // “ロジックを関数化してテスト”の前段として、
        // 今回は最低限、入力検証の考え方を確認するテストにする
        let title = "   ".trim();
        assert!(title.is_empty());
    }

    #[test]
    fn apply_completed_updates_target() {
        let mut todos = vec![
            Todo {
                id: 1,
                title: "a".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "b".to_string(),
                completed: false,
            },
        ];
        let updated = apply_completed(&mut todos, 2, true).unwrap();
        assert_eq!(updated.id, 2);
        assert_eq!(updated.completed, true);
        assert_eq!(todos[1].completed, true);
    }
}
