UPDATE users SET first_name = ''  WHERE first_name IS NULL;
UPDATE users SET last_name  = ''  WHERE last_name  IS NULL;
UPDATE users SET role       = 'user' WHERE role       IS NULL;

ALTER TABLE users
  ALTER COLUMN first_name SET DEFAULT '',
  ALTER COLUMN last_name  SET DEFAULT '',
  ALTER COLUMN role       SET DEFAULT 'user';

ALTER TABLE users
  ALTER COLUMN first_name SET NOT NULL,
  ALTER COLUMN last_name  SET NOT NULL,
  ALTER COLUMN role       SET NOT NULL,
  ALTER COLUMN username   SET NOT NULL;

ALTER TABLE users add CONSTRAINT users_username_key UNIQUE (username);