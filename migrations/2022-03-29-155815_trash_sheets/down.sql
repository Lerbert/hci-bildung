DROP FUNCTION delete_expired_sheets;

ALTER TABLE sheets DROP COLUMN trashed;
