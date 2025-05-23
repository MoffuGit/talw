CREATE TABLE IF NOT EXISTS members (
  id binary(16) NOT NULL UNIQUE,
  user_id binary(16) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id),
  server_id binary(16) NOT NULL,
  FOREIGN KEY (server_id) REFERENCES servers (id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
