CREATE TABLE IF NOT EXISTS channel_messages (
  id BINARY(16) NOT NULL UNIQUE,
  channel_id BINARY(16) NOT NULL,
  FOREIGN KEY (channel_id) REFERENCES channels (id),
  sender_id BINARY(16) NOT NULL,
  FOREIGN KEY (sender_id) REFERENCES members (id),
  message_reference BINARY(16),
  FOREIGN KEY (message_reference) REFERENCES channel_messages (id) ON DELETE
  SET
    NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    edited_timestamp TIMESTAMP NULL ON UPDATE CURRENT_TIMESTAMP,
    pinned BOOLEAN DEFAULT FALSE,
    mention_everyone BOOLEAN DEFAULT FALSE
);
