-- Add updated_at column (SQLite-safe).
ALTER TABLE todos
ADD COLUMN updated_at TEXT;

-- Backfill existing rows.
UPDATE todos
SET updated_at = datetime('now')
WHERE updated_at IS NULL;
