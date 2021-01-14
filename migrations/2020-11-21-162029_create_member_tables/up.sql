-- Your SQL goes here
CREATE TABLE user_group_members (
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  "group" INTEGER REFERENCES "groups" ON DELETE CASCADE,
  PRIMARY KEY ("user", "group")
);

CREATE TABLE group_group_members (
  parent INTEGER REFERENCES "groups" ON DELETE CASCADE,
  child INTEGER REFERENCES "groups" ON DELETE CASCADE,
  PRIMARY KEY (parent, child)
);