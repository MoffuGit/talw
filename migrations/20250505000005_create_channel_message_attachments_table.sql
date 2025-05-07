CREATE TABLE IF NOT EXISTS channel_messages_attachments (
  message_id BINARY(16) NOT NULL,
  FOREIGN KEY (message_id) REFERENCES channel_messages (id),
  attachment_id BINARY(16) NOT NULL,
  FOREIGN KEY (attachment_id) REFERENCES attachments (id)
);
