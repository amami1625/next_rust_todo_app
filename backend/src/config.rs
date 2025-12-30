use std::env;

/// アプリケーション設定
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,  // データベース接続文字列
    pub host: String,          // サーバーのホスト
    pub port: u16,             // サーバーのポート
}

impl Config {
    /// 環境変数から設定を読み込む
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            // DATABASE_URL 環境変数を取得（必須）
            database_url: env::var("DATABASE_URL")?,
            // HOST 環境変数を取得（デフォルト: 0.0.0.0）
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            // PORT 環境変数を取得（デフォルト: 8080）
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
        })
    }
}
