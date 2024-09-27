CREATE TABLE IF NOT EXISTS channels (
  id binary(16) NOT NULL UNIQUE,
  name varchar(30) NOT NULL,
  channel_type enum ("TEXT") DEFAULT "TEXT",
  server_id binary(16) NOT NULL,
  FOREIGN KEY (server_id) REFERENCES servers (id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
