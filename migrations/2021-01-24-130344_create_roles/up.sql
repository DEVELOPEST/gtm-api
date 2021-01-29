-- Your SQL goes here
CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_role_members (
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  role INTEGER REFERENCES roles ON DELETE CASCADE,
  PRIMARY KEY ("user", role)
);