CREATE TABLE IF NOT EXISTS servers (
  id binary(16) NOT NULL UNIQUE,
  name varchar(30) NOT NULL,
  invite_code binary(16) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
