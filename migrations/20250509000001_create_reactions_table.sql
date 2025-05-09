CREATE TABLE IF NOT EXISTS reactions (
  id binary(16) NOT NULL UNIQUE,
  name VARCHAR(25) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  counter INT UNSIGNED NOT NULL DEFAULT 0,
  message_id binary(16) NOT NULL,
  FOREIGN KEY (message_id) REFERENCES channel_messages (id)
);
