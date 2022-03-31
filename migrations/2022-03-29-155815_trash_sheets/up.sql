ALTER TABLE sheets ADD COLUMN trashed TIMESTAMPTZ NULL;

-- delete all trashed sheets older than 30 days
CREATE FUNCTION delete_expired_sheets()
RETURNS VOID
LANGUAGE sql
AS $$
    DELETE FROM sheets
    WHERE trashed IS NOT NULL AND trashed < NOW() - INTERVAL '30 days';
$$;
