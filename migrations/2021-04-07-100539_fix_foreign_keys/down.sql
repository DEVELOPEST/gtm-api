-- This file should undo anything in `up.sql`
ALTER TABLE repositories
    DROP CONSTRAINT fk_repositories_groups;