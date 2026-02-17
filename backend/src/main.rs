mod app;
mod error;
mod models;
mod routes;
mod state;
mod store;

use sqlx::{self, SqlitePool};
use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://db/todos.db").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let state = AppState::new(pool);
    let app = app::app(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
