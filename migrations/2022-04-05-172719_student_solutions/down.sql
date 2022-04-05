DROP FUNCTION delete_expired_solutions;

DROP TABLE solutions;

CREATE FUNCTION update_changed_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.changed = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER sheet_changed BEFORE UPDATE ON sheets
FOR EACH ROW EXECUTE PROCEDURE update_changed_timestamp();
