use serde::{Deserialize, Serialize};
use validator::Validate;

// DB から取得する User の構造体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

// ユーザー登録リクエスト
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(email(message = "メールアドレスの形式が正しくありません"))]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上で入力してください"))]
    pub password: String,
}

// ログインリクエスト
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// ログイン成功時のレスポンス（token を返す）
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
