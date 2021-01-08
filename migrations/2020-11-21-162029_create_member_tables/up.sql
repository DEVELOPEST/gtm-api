-- Your SQL goes here
CREATE TABLE user_group_members (
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  "group" INTEGER REFERENCES groups ON DELETE CASCADE,
  PRIMARY KEY ("user", "group")
);

CREATE TABLE group_repository_members (
  repository INTEGER REFERENCES repositories ON DELETE CASCADE,
  "group" INTEGER REFERENCES groups ON DELETE CASCADE,
  PRIMARY KEY (repository, "group")
);