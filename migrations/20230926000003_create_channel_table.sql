CREATE TABLE IF NOT EXISTS channels (
  id binary(36) NOT NULL,
  name varchar(30) NOT NULL,
  type ENUM("TEXT") DEFAULT "TEXT",

  server_id binary(36) NOT NULL,
  FOREIGN KEY (server_id) REFERENCES servers(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
