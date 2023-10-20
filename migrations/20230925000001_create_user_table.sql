SET sql_generate_invisible_primary_key=ON;

CREATE TABLE IF NOT EXISTS users (
  id         binary(16) NOT NULL UNIQUE,
  username   varchar(30) NOT NULL UNIQUE,
  password   varchar(60) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
