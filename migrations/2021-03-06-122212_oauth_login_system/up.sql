-- Your SQL goes here
DROP TABLE IF EXISTS user_group_members;

CREATE TABLE login_type
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL
);

INSERT INTO login_type (id, name)
VALUES (1, 'oauth_github'),
       (2, 'oauth_gitlab'),
       (3, 'oauth_microsoft'),
       (4, 'oauth_google'),
       (5, 'oauth_discord');

CREATE TABLE "login"
(
    id            SERIAL PRIMARY KEY,
    "user"        INTEGER NOT NULL,
    login_type    INTEGER NOT NULL,
    token         TEXT    NOT NULL,
    refresh_token TEXT    NULL,
    exp           BIGINT  NULL,
    CONSTRAINT ak_user_login_type UNIQUE ("user", login_type)
);

ALTER TABLE users
    RENAME COLUMN email
        TO username;

ALTER TABLE users
    ALTER COLUMN password DROP NOT NULL;