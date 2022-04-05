-- we control this in application logic now
DROP TRIGGER sheet_changed ON sheets;
DROP FUNCTION update_changed_timestamp();

CREATE TABLE solutions (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    sheet_id uuid REFERENCES sheets ON UPDATE CASCADE ON DELETE SET NULL,
    sheet_version TIMESTAMPTZ NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES users ON UPDATE CASCADE ON DELETE RESTRICT,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    changed TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    trashed TIMESTAMPTZ NULL,
    solution JSONB NOT NULL,
    UNIQUE (sheet_id, sheet_version, owner_id)
);

-- delete all trashed solutions older than 30 days
CREATE FUNCTION delete_expired_solutions()
RETURNS VOID
LANGUAGE sql
AS $$
    DELETE FROM solutions
    WHERE trashed IS NOT NULL AND trashed < NOW() - INTERVAL '30 days';
$$;
