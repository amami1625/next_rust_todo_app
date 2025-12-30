mod config;
mod db;
mod handlers;
mod models;
mod modules;
mod routes;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ロギングの初期化
    modules::logging::init();

    // .env ファイルから環境変数を読み込み（開発環境用）
    dotenv::dotenv().ok();

    // 設定を環境変数から読み込む
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // データベース接続プールを作成
    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // マイグレーションを実行
    tracing::info!("Running migrations...");
    db::run_migrations(&pool).await?;
    tracing::info!("Migrations completed");

    // サーバーアドレスを設定
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server starting on {}", addr);

    // API ルーターを作成
    let app = routes::create_router(pool);

    // TCP リスナーを作成
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    // Axum サーバーを起動
    axum::serve(listener, app).await?;

    Ok(())
}
