-- Your SQL goes here
CREATE TABLE group_accesses (
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  "group" INTEGER REFERENCES groups ON DELETE CASCADE,
  access_level_recursive boolean NOT NULL ,
  PRIMARY KEY ("user", "group")
);