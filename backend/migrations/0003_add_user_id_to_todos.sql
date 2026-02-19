-- todos テーブルに user_id カラムを追加
-- REFERENCES users(id) で外部キー制約を設定
-- ON DELETE CASCADE でユーザー削除時に紐づく Todo も削除される
ALTER TABLE todos
  ADD COLUMN user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE;
