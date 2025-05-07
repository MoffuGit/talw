CREATE TABLE IF NOT EXISTS messages_mentions (
    message_id binary(16) NOT NULL,
    FOREIGN KEY (message_id) REFERENCES channel_messages (id),
    member_id binary(16) NOT NULL,
    FOREIGN KEY (member_id) REFERENCES members (id)
);
