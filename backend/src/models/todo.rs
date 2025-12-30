use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// データベースから取得する Todo の完全な形
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Todo {
    pub id: Uuid,                      // 一意の識別子
    pub title: String,                 // タイトル
    pub description: Option<String>,   // 説明（任意）
    pub completed: bool,               // 完了状態
    pub created_at: DateTime<Utc>,     // 作成日時
    pub updated_at: DateTime<Utc>,     // 更新日時
}

// 新しい Todo を作成するときに受け取るデータ
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,                 // タイトルのみ必須
    pub description: Option<String>,   // 説明は任意
}

// Todo を更新するときに受け取るデータ
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,         // タイトルを変更する場合
    pub description: Option<String>,   // 説明を変更する場合
    pub completed: Option<bool>,       // 完了状態を変更する場合
}
