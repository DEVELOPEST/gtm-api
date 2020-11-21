-- Your SQL goes here
CREATE TABLE user_git_group_members (
  "user" INTEGER REFERENCES users ON DELETE CASCADE,
  git_group INTEGER REFERENCES git_groups ON DELETE CASCADE,
  PRIMARY KEY ("user", git_group)
);

CREATE TABLE git_group_repository_members (
  repository INTEGER REFERENCES repositories ON DELETE CASCADE,
  git_group INTEGER REFERENCES git_groups ON DELETE CASCADE,
  PRIMARY KEY (repository, git_group)
);