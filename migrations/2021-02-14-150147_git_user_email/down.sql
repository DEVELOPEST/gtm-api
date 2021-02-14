UPDATE commits
SET email = git_user_name || ' <' || commits.email ||'>';

ALTER TABLE commits
DROP COLUMN git_user_name;