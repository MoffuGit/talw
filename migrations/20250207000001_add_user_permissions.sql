CREATE TABLE IF NOT EXISTS user_permissions (
  user_id binary(16) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id),
  token TEXT
);
