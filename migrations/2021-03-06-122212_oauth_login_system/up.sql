-- Your SQL goes here
DROP TABLE IF EXISTS user_group_members;

CREATE TABLE login_types
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL
);

INSERT INTO login_types (id, name)
VALUES (1, 'oauth_token_github'),
       (2, 'oauth_bearer_gitlab'),
       (3, 'oauth_bearer_microsoft');

CREATE TABLE logins
(
    id            SERIAL PRIMARY KEY,
    "user"        INTEGER NOT NULL,
    login_type    INTEGER NOT NULL,
    identity_hash TEXT    NOT NULL,
    token         TEXT    NOT NULL,
    refresh_token TEXT    NULL,
    exp           BIGINT  NULL,
    CONSTRAINT ak_user_login_type UNIQUE ("user", login_type),
    CONSTRAINT fk_user_user_id FOREIGN KEY ("user") REFERENCES "users"(id) ON DELETE CASCADE,
    CONSTRAINT fk_login_type_login_type_id FOREIGN KEY (login_type) REFERENCES login_types(id) ON DELETE RESTRICT
);

ALTER TABLE users
    RENAME COLUMN email
        TO username;

ALTER TABLE users RENAME CONSTRAINT users_email_key TO users_username_key;

ALTER TABLE users
    ALTER COLUMN password DROP NOT NULL;
