-- Your SQL goes here

ALTER TABLE repositories
    ADD CONSTRAINT fk_repositories_groups FOREIGN KEY ("group") REFERENCES GROUPS (id);