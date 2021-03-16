-- This file should undo anything in `up.sql`
DELETE FROM login_types
WHERE id IN (4, 5);