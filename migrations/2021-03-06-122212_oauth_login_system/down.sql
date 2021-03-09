-- This file should undo anything in `up.sql`
CREATE TABLE user_group_members
(
    "user"  INTEGER REFERENCES users ON DELETE CASCADE,
    "group" INTEGER REFERENCES "groups" ON DELETE CASCADE,
    PRIMARY KEY ("user", "group")
);

DROP TABLE IF EXISTS logins;

DROP TABLE IF EXISTS login_types;

ALTER TABLE users
    RENAME COLUMN username
        TO email;

ALTER TABLE users RENAME CONSTRAINT users_username_key TO users_email_key;

ALTER TABLE users
    ALTER COLUMN password SET NOT NULL;