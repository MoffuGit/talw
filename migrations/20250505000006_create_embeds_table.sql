CREATE TABLE IF NOT EXISTS embeds (
  id binary(16) NOT NULL UNIQUE,
  url VARCHAR(2048) NOT NULL,
  data JSON NOT NULL
);
