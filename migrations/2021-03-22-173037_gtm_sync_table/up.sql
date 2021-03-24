-- Your SQL goes here

CREATE TABLE IF NOT EXISTS sync_client_type
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sync_clients
(
    id               SERIAL PRIMARY KEY,
    base_url         TEXT    NOT NULL,
    api_key          TEXT    NOT NULL,
    sync_client_type INTEGER NOT NULL REFERENCES sync_client_type (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

INSERT INTO sync_client_type (id, name)
VALUES (1, 'public'),
       (2, 'private');

AlTER TABLE repositories
    DROP COLUMN sync_url;

ALTER TABLE repositories
    DROP COLUMN access_token;

ALTER TABLE repositories
    ADD COLUMN sync_client INTEGER NULL REFERENCES sync_clients (id) ON UPDATE CASCADE ON DELETE SET NULL;
