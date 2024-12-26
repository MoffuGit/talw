CREATE TABLE IF NOT EXISTS banners (
  id binary(16) NOT NULL UNIQUE,
  image_url varchar(100),
  primary_color binary(6),
  accent_color binary(6),
  about varchar(190),
  user_id binary(16) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id)
);
