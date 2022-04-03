CREATE TYPE role AS ENUM (
    'teacher',
    'student'
);

CREATE TABLE roles (
  user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
  role role NOT NULL,
  PRIMARY KEY (user_id, role)
);

-- all current users are teachers
INSERT INTO roles (user_id, role)
SELECT id, 'teacher'
FROM users;
