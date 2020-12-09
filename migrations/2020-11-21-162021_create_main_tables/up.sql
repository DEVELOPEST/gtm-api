-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  hash TEXT NOT NULL
);

CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  access_token TEXT NOT NULL,
  added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE git_groups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  url TEXT NOT NULL,
  sync_url TEXT NOT NULL,
  access_token TEXT NOT NULL,
  added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE commits (
  id SERIAL PRIMARY KEY,
  repository_id INTEGER NOT NULL REFERENCES repositories ON DELETE CASCADE,
  hash TEXT NOT NULL,
  message TEXT NOT NULL,
  email TEXT NOT NULL,
  branch TEXT NOT NULL,
  timestamp BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE files (
  id SERIAL PRIMARY KEY,
  commit INTEGER NOT NULL REFERENCES commits ON DELETE CASCADE,
  path TEXT NOT NULL,
  status TEXT NOT NULL,
  time BIGINT NOT NULL DEFAULT 0,
  lines_added BIGINT NOT NULL DEFAULT 0,
  lines_deleted BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE timeline (
  id SERIAL PRIMARY KEY,
  file INTEGER NOT NULL REFERENCES files ON DELETE CASCADE,
  timestamp BIGINT NOT NULL DEFAULT 0,
  time BIGINT NOT NULL DEFAULT 0
);