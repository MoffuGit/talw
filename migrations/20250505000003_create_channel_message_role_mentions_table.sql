CREATE TABLE IF NOT EXISTS messages_role_mentions (
    message_id binary(16) NOT NULL,
    FOREIGN KEY (message_id) REFERENCES channel_messages (id),
    role_id binary(16) NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles (id)
);
