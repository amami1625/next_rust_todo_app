use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::AppError,
    models::user::{AuthResponse, LoginUser, RegisterUser, User},
    state::AppState,
};

// JWT のペイロード
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // sub = subject（このトークンが誰のものか）= ユーザーの ID
    sub: i64,
    // exp = expiration（有効期限）= Unix タイムスタンプ（秒）
    exp: usize,
}

// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // バリデーション（メールアドレスの形式・パスワードの長さ）
    payload.validate()?;

    // パスワードをハッシュ化して保存する
    let password_hash = hash(&payload.password, DEFAULT_COST).map_err(|e| {
        eprintln!("[bcrypt error] {e:?}");
        AppError::Internal
    })?;

    // DB にユーザーを登録する
    // メールアドレスが重複している場合は UNIQUE 制約でエラーになる
    let row = sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        // PostgreSQL のエラーコード 23505 = unique_violation（重複）
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict;
            }
        }
        eprintln!("[sqlx error][register] {e:?}");
        AppError::Internal
    })?;

    let token = create_token(row, &state.jwt_secret)?;

    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}

// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<AuthResponse>, AppError> {
    // メールアドレスでユーザーを検索する
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at::text AS created_at, updated_at::text AS updated_at FROM users WHERE email = $1",
    )
    .bind(&payload.email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("[sqlx error][login] {e:?}");
        AppError::Internal
    })?;

    // ユーザーが見つからない場合も「メールアドレスまたはパスワードが間違い」と返す
    // 「メールアドレスが存在しない」と返すと、登録済みアドレスが特定できてしまうため
    let Some(user) = user else {
        return Err(AppError::Unauthorized);
    };

    // 入力パスワードとハッシュを比較する
    // verify() は平文パスワードと bcrypt ハッシュを安全に比較してくれる
    let is_valid = verify(&payload.password, &user.password_hash).map_err(|e| {
        eprintln!("[bcrypt error] {e:?}");
        AppError::Internal
    })?;

    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse { token }))
}

// JWT トークンを生成する共通処理
fn create_token(user_id: i64, secret: &str) -> Result<String, AppError> {
    // 現在の Unix タイムスタンプ（秒）を取得
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // 有効期限: 現在時刻 + 7日間
    let exp = now + 60 * 60 * 24 * 7;

    let claims = Claims {
        sub: user_id,
        exp,
    };

    // HS256 アルゴリズム（HMAC-SHA256）でトークンに署名する
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        eprintln!("[jwt error] {e:?}");
        AppError::Internal
    })
}
