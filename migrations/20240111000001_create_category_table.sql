CREATE TABLE IF NOT EXISTS categories (
  id binary(16) NOT NULL UNIQUE,
  name varchar(30) NOT NULL,
  server_id binary(16) NOT NULL,
  FOREIGN KEY (server_id) REFERENCES servers (id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
