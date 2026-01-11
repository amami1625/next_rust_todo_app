use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Clone, Serialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

fn sample_todos() -> Vec<Todo> {
    vec![
        Todo { id: 1, title: "First todo".to_string(), completed: false },
        Todo { id: 2, title: "Second todo".to_string(), completed: true },
    ]
}

async fn get_todos() -> Json<Vec<Todo>> {
    // まずは固定データでOK（Stage A）
    Json(sample_todos())
}

#[tokio::main]
async fn main() {
    // ルータ
    let app = Router::new()
        .route("/todos", get(get_todos))
        // 開発用に広めに許可（あとで絞る）
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = "0.0.0.0:3001";
    println!("Backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_todos_returns_two_items() {
        let todos = sample_todos();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, 1);
    }
}
