CREATE TABLE IF NOT EXISTS profiles (
  id binary(16) NOT NULL UNIQUE,
  name varchar(30) NOT NULL,
  image_url varchar(100),
  user_id binary(16) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id)
);
