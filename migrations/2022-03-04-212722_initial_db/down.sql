-- Triggers

DROP TRIGGER sheet_changed ON sheets;
DROP FUNCTION update_changed_timestamp();

-- Tables

DROP TABLE sessions;
DROP TABLE sheets;
DROP TABLE users;

DROP EXTENSION pgcrypto;
