ALTER TABLE channels
    MODIFY COLUMN
        channel_type ENUM(
            "TEXT",
            "VOICE",
            "ANNOUNCEMENTS",
            "RULES"
        )
