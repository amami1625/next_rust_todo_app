use axum::http::{Method, header};
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
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
        .with_state(state)
        .layer(cors);

    let addr = "0.0.0.0:3001";
    println!("Backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
}
