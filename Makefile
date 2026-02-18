# 全サービスを起動
up:
	docker compose up -d

# 全サービスを停止
down:
	docker compose down

# バックエンドをリビルドして再起動
rebuild:
	docker compose build backend && docker compose up -d backend

# ログを表示（Ctrl+C で終了）
logs:
	docker compose logs -f

# バックエンドのログだけ表示
logs-back:
	docker compose logs -f backend

# フロントエンドのログだけ表示
logs-front:
	docker compose logs -f frontend

# コンテナの状態を確認
ps:
	docker compose ps

# Rust のテスト（ローカル実行）
test:
	cd backend && cargo test

# Rust のフォーマット
fmt:
	cd backend && cargo fmt

# Rust の lint
clippy:
	cd backend && cargo clippy -- -D warnings

# コンテナとボリュームを削除
clean:
	docker compose down -v
