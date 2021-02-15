ALTER TABLE commits
ADD COLUMN git_user_name TEXT NULL;

UPDATE commits
SET git_user_name = substring(email from '^(.*)\s+<.*>$');

ALTER TABLE commits
ALTER COLUMN git_user_name SET NOT NULL;

UPDATE commits
SET email = substring(email from '^.*\s+<(.*)>$');