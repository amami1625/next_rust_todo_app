mod app;
mod error;
mod models;
mod handler;
mod state;
mod store;

use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("dev-secret-key".to_string());
    let state = AppState::new(pool, jwt_secret);
    let app = app::app(state);

    // HOST 環境変数がなければ 127.0.0.1 を使う（ローカル開発用）
    // Docker では HOST=0.0.0.0 を設定する
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
