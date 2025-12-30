.PHONY: up down build logs clean test help

# Docker Compose を起動
up:
	docker compose up -d

# Docker Compose を停止
down:
	docker compose down

# Docker イメージをビルド
build:
	docker compose build

# ログを表示
logs:
	docker compose logs -f

# コンテナとボリュームを削除
clean:
	docker compose down -v

# テストを実行
test:
	@if [ -d backend ]; then cd backend && cargo test; else cargo test; fi

# ヘルプを表示
help:
	@echo "利用可能なコマンド:"
	@echo "  make up      - Docker Compose を起動"
	@echo "  make up-d    - Docker Compose をバックグラウンドで起動"
	@echo "  make down    - Docker Compose を停止"
	@echo "  make build   - Docker イメージをビルド"
	@echo "  make logs    - ログを表示"
	@echo "  make test    - テストを実行"
	@echo "  make clean   - コンテナとボリュームを削除"
	@echo "  make help    - このヘルプを表示"
