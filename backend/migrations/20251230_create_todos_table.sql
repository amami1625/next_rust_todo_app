-- マイグレーション: todos テーブルを作成
-- UUID 拡張を有効化（UUID を使うために必要）
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- todos テーブルの作成
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- 自動生成される一意のID
    title TEXT NOT NULL,                              -- タイトル（必須）
    description TEXT,                                 -- 説明（任意）
    completed BOOLEAN NOT NULL DEFAULT false,         -- 完了状態（デフォルト: 未完了）
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),    -- 作成日時（自動設定）
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()     -- 更新日時（自動設定）
);

-- updated_at を自動更新するためのトリガー関数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();  -- 更新時に現在時刻を設定
    RETURN NEW;
END;
$$ language 'plpgsql';

-- トリガーを todos テーブルに設定
CREATE TRIGGER update_todos_updated_at
    BEFORE UPDATE ON todos
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- インデックスの作成（検索を高速化）
CREATE INDEX idx_todos_completed ON todos(completed);
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);
