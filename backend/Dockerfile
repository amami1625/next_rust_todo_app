FROM rust:1.88-slim AS builder

WORKDIR /app

# 依存関係のキャッシュのため、先に Cargo ファイルだけコピーしてビルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# ソースコードをコピーして本番ビルド
COPY . .
RUN touch src/main.rs && cargo build --release

# 実行用の軽量イメージ
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app

CMD ["backend"]
