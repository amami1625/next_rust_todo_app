use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// データベース接続プールを作成
///
/// database_url: PostgreSQL の接続文字列（例: postgres://user:pass@localhost/db）
/// 戻り値: 接続プール (PgPool)
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    // 接続プールの設定
    PgPoolOptions::new()
        .max_connections(5)                          // 最大接続数
        .acquire_timeout(Duration::from_secs(3))     // 接続取得のタイムアウト
        .connect(database_url)                       // データベースに接続
        .await
}

/// データベースマイグレーションを実行
///
/// pool: データベース接続プール
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    // migrations フォルダ内の SQL ファイルを実行
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}
