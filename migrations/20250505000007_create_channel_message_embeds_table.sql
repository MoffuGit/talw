CREATE TABLE IF NOT EXISTS channel_messages_embeds (
  message_id binary(16) NOT NULL,
  FOREIGN KEY (message_id) REFERENCES channel_messages (id),
  embeds_id binary(16) NOT NULL,
  FOREIGN KEY (embeds_id) REFERENCES embeds (id)
);
