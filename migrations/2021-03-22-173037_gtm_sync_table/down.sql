-- This file should undo anything in `up.sql`

ALTER TABLE repositories
    DROP COLUMN sync_client;

AlTER TABLE repositories
    ADD COLUMN sync_url TEXT NOT NULL DEFAULT '';

ALTER TABLE repositories
    ADD COLUMN access_token TEXT NOT NULL DEFAULT '';

DROP TABLE IF EXISTS sync_clients;

DROP TABLE IF EXISTS sync_client_type;

