-- Add created_at column (SQLite: avoid non-constant DEFAULT in ALTER TABLE).
ALTER TABLE todos
ADD COLUMN created_at TEXT;

-- Backfill existing rows (set now for rows that are NULL).
UPDATE todos
SET created_at = datetime('now')
WHERE created_at IS NULL;
