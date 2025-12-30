# Todo App - Next.js + Rust

Next.js 15 と Rust で構築する Todo アプリケーション

## 技術スタック

### Frontend
- Next.js 15 (App Router)
- TypeScript
- Tailwind CSS

### Backend
- Rust
- Axum (Web Framework)
- PostgreSQL
- SQLx (Database Driver)

## 開発環境のセットアップ

### 必要なもの
- Docker & Docker Compose

### 起動方法

```bash
# 全サービスを起動
docker-compose up

# バックグラウンドで起動
docker-compose up -d

# ログを確認
docker-compose logs -f

# 停止
docker-compose down
```

### アクセス
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- PostgreSQL: localhost:5432

## API エンドポイント

- `GET /api/todos` - Todo 一覧を取得
- `GET /api/todos/:id` - 特定の Todo を取得
- `POST /api/todos` - 新しい Todo を作成
- `PUT /api/todos/:id` - Todo を更新
- `DELETE /api/todos/:id` - Todo を削除

## 学習ポイント

このプロジェクトでは以下を学べます：
- Rust での REST API 開発
- PostgreSQL との接続と CRUD 操作
- Next.js からの API 呼び出し
- Docker での開発環境構築
