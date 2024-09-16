CREATE TABLE IF NOT EXISTS threads (
    id binary(16) NOT NULL UNIQUE,
    name varchar(30) NOT NULL,

    channel_id binary(16) NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels (id),

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
