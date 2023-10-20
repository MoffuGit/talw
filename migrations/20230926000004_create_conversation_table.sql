CREATE TABLE IF NOT EXISTS conversations (
  id binary(16) NOT NULL UNIQUE,
  user_one_id binary(16) NOT NULL,
  FOREIGN KEY (user_one_id) REFERENCES users(id),
  user_two_id binary(16) NOT NULL,
  FOREIGN KEY (user_two_id) REFERENCES users(id)
);
