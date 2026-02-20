use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};

// JWT のペイロード（auth.rs の Claims と同じ構造）
// decode するときもこの構造体に当てはめて取り出す
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}

// 認証ミドルウェア
// このミドルウェアを通過したリクエストだけがハンドラーに到達できる
pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Authorization ヘッダーを取り出す
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok()); // ヘッダー値を &str に変換（失敗したら None）

    // ヘッダーがなければ 401 を返す
    let Some(auth_header) = auth_header else {
        return Err(AppError::Unauthorized);
    };

    // 2. "Bearer " プレフィックスを取り除いてトークン本体だけ取り出す
    let Some(token) = auth_header.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized);
    };

    // 3. JWT トークンを検証して Claims（ペイロード）を取り出す
    //    - 署名が正しいか（改ざんされていないか）
    //    - 有効期限が切れていないか
    //    の2点を検証
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        eprintln!("[jwt decode error] {e:?}");
        AppError::Unauthorized
    })?;

    // 4. user_id をリクエストの「拡張（Extension）」に追加する
    req.extensions_mut().insert(token_data.claims.sub);

    // 5. 次の処理（ハンドラー）に進む
    Ok(next.run(req).await)
}
