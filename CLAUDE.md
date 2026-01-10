# CLAUDE.md

このファイルは、このリポジトリで作業する Claude Code (claude.ai/code) にガイダンスを提供します。

**重要: このプロジェクトでは、すべてのやり取りを日本語で行ってください。コード内のコメントも日本語で記述してください。**

## プロジェクト概要

Rust での REST API 開発を学習するための Todo アプリケーションプロジェクトです：
- **バックエンド**: Rust + Axum (REST API) - 実装済み
- **フロントエンド**: Next.js 16 + TypeScript - 未実装
- **データベース**: PostgreSQL
- **インフラ**: Docker Compose

完全な CRUD 操作を通じて Rust API 開発を学ぶことを目的としています。

## 開発コマンド

### Docker 環境（推奨）

```bash
# 全サービスを起動（PostgreSQL + バックエンド）
make up

# ログを表示
make logs

# サービスを停止
make down

# イメージを再ビルド
make build

# コンテナとボリュームを削除
make clean
```

### Rust 開発

```bash
# テストを実行
make test
# または: cd backend && cargo test

# コードをフォーマット
make fmt
# または: cd backend && cargo fmt

# コードを lint
make clippy
# または: cd backend && cargo clippy -- -D warnings

# 特定のテストを実行
cd backend && cargo test test_name

# 出力付きでテストを実行
cd backend && cargo test -- --nocapture
```

### API テスト

Docker で起動すると、バックエンドは `http://localhost:8080` で動作します。

エンドポイント例:
- `GET /api/todos` - Todo 一覧を取得
- `POST /api/todos` - Todo を作成 (body: `{"title": "...", "description": "..."}`)
- `PUT /api/todos/:id` - Todo を更新
- `DELETE /api/todos/:id` - Todo を削除

## アーキテクチャ

### バックエンドの構造（レイヤードアーキテクチャ）

```
src/
├── main.rs           # エントリーポイント: ロギング、設定、DB プール、サーバー起動
├── config.rs         # 環境変数の設定
├── routes.rs         # ルート定義（URL → ハンドラーのマッピング）
├── handlers/         # リクエストハンドラー（ビジネスロジック）
│   └── todos.rs      # Todo API の CRUD ハンドラー
├── models/           # データ構造
│   └── todo.rs       # Todo, CreateTodo, UpdateTodo 構造体
├── db/               # データベース層
│   └── mod.rs        # 接続プール、マイグレーション
└── modules/          # ユーティリティモジュール
    └── logging.rs    # ロギング初期化
```

### 主要な設計パターン

**State パターン（依存性注入）**:
- データベース接続プール（`PgPool`）は Axum の State を介して全ハンドラーで共有される
- `routes.rs` で `.with_state(pool)` により登録
- ハンドラーで `State(pool): State<PgPool>` として抽出

**Extractor パターン**:
Axum はエクストラクターを使ってリクエストデータを解析:
- `State(pool): State<PgPool>` - 共有状態を抽出
- `Json(payload): Json<CreateTodo>` - JSON ボディを解析
- `Path(id): Path<Uuid>` - URL パラメータを抽出

**エラーハンドリング**:
- ハンドラーは `Result<T, (StatusCode, String)>` を返す
- `?` 演算子でエラーを伝播
- エラーは返す前に `tracing::error!` でログに記録

### データベース

**接続プール**:
- `db/create_pool()` で最大 5 接続に設定
- リクエスト間で接続を効率的に再利用
- Axum State を介してプールを共有

**マイグレーション**:
- `backend/migrations/` に配置
- サーバー起動時に `db::run_migrations()` で自動実行
- SQLx のマイグレーションシステムを使用

**スキーマ**:
- `todos` テーブルは UUID を主キーとする
- `id`, `created_at`, `updated_at` は自動生成
- トリガーが更新時に `updated_at` を自動更新

### データフローの例（Todo 作成）

1. クライアントが JSON ボディ付きで `POST /api/todos` を送信
2. Axum が `handlers::todos::create_todo` にルーティング
3. `Json(payload)` がボディを `CreateTodo` 構造体に抽出
4. `State(pool)` がデータベース接続プールを抽出
5. ハンドラーが `.bind()` でパラメータを設定して SQL INSERT を実行
6. `RETURNING` 句が作成されたレコードを返す
7. ハンドラーが `(StatusCode::CREATED, Json(todo))` を返す

## コードスタイル規約

### 使用している Rust パターン

**Async/Await**:
- 全てのハンドラーと DB 操作は非同期
- Tokio ランタイムを使用（`#[tokio::main]`）

**型安全性**:
- SQLx がコンパイル時に SQL をチェック
- `sqlx::query_as::<_, Todo>()` で SQL 結果を構造体にマッピング
- `$1, $2` のパラメータ化クエリで SQL インジェクションを防止

**Option 型**:
- `description` のような null 許容フィールドには `Option<String>`
- `UpdateTodo` では `Option<Option<String>>` を使って区別:
  - `None` = フィールドを更新しない
  - `Some(None)` = null に設定
  - `Some(Some(value))` = 値を設定

### テスト

テストは同じファイル内に配置:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // テストコード
    }
}
```

現在のテストカバレッジ: `modules::logging` モジュールのみ。

## 環境変数

必要な環境変数（`.env.example` を参照）:
- `DATABASE_URL` - PostgreSQL 接続文字列
- `HOST` - サーバーのバインドアドレス（デフォルト: 0.0.0.0）
- `PORT` - サーバーのポート（デフォルト: 8080）
- `RUST_LOG` - ログレベル（デフォルト: debug）

Docker では、これらは `compose.yaml` で設定されます。

## フロントエンド（未実装）

frontend ディレクトリは存在しますが、実装はありません。実装する際は:
- Next.js 16 と App Router を使用
- API 呼び出しは `http://localhost:8080/api/*` へ
- 上記で文書化された API エンドポイントに従う

## 重要な注意事項

**モジュール構成**:
- 新しいユーティリティモジュールは `src/modules/` に配置
- `src/modules/mod.rs` に `pub mod module_name;` を追加することを忘れずに
- main.rs では `use modules::module_name;` でインポート

**パターンマッチングエクストラクター**:
- `State(value)` と `Json(value)` は関数呼び出しではない
- ラッパー型から内部の値を取り出すパターンマッチング
- これは Rust の分配構文

**State 管理**:
- State は `routes::create_router()` で一度だけ設定
- 全てのハンドラーが自動的に同じ共有状態を受け取る
- 各ハンドラーに手動で state を渡す必要はない

**エラーメッセージ**:
- エラーを返す前に必ず `tracing::error!` でログに記録
- レスポンスにはユーザーフレンドリーなエラーメッセージを返す
- 内部エラーは完全なデバッグ情報 `{:?}` でログに記録

## ドキュメント

- アーキテクチャの詳細: `docs/backend/ARCHITECTURE.md`（Mermaid 図を含む）
- API ドキュメント: README.md の API エンドポイントセクションを参照
