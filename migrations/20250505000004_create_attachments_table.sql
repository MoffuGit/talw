CREATE TABLE IF NOT EXISTS attachments (
  id binary(16) NOT NULL UNIQUE,
  filename varchar(100) NOT NULL,
  url varchar(100) NOT NULL
);
