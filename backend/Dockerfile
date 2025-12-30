# ビルドステージ
FROM rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /app

# 依存関係のみを先にビルド（キャッシュ効率化）
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# ソースコードをコピー
COPY . .

# アプリケーションをビルド（依存関係は既にキャッシュ済み）
RUN touch src/main.rs && cargo build --release

# 実行ステージ（軽量イメージ）
FROM debian:bookworm-slim

# 必要なランタイムライブラリをインストール
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを設定
WORKDIR /app

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/todo-backend /app/todo-backend

# マイグレーションファイルをコピー
COPY --from=builder /app/migrations /app/migrations

# ポート8080を公開
EXPOSE 8080

# アプリケーションを実行
CMD ["/app/todo-backend"]
