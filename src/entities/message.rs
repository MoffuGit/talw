use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use log::debug;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::open_graph::OpenGraph;

use super::member::Member;
use super::role::Role;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
        use super::Error;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Store)]
pub struct ChannelMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub thread_id: Option<Uuid>,
    pub sender: Member,
    pub message_reference: Option<Box<ChannelMessage>>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    //impl
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<Member>,
    pub mentions_roles: Vec<Role>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Vec<Reaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Store)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Store)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Embed {
    pub id: Uuid,
    pub url: String,
    pub data: JsonValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Store)]
pub struct Reaction {
    pub id: Uuid,
    pub message_id: Uuid,
    pub name: String,
    pub counter: u32,
    pub me: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct SqlReaction {
    id: Uuid,
    message_id: Uuid,
    name: Vec<u8>,
    counter: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct SqlChannelMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub thread_id: Option<Uuid>,
    pub sender_id: Uuid,
    pub message_reference: Option<Uuid>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub pinned: bool,
    pub mention_everyone: bool,
}

#[cfg(feature = "ssr")]
impl ChannelMessage {
    pub async fn mention_everyone(message_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE channel_messages ch SET ch.mention_everyone = TRUE WHERE ch.id = ?")
            .bind(message_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add_embed(
        message_id: Uuid,
        op: OpenGraph,
        url: String,
        pool: &MySqlPool,
    ) -> Result<Embed, Error> {
        let id = Uuid::new_v4();
        let data = serde_json::to_value(op).unwrap();
        sqlx::query("INSERT INTO embeds (id, url, data) VALUES (?, ?, ?)")
            .bind(id)
            .bind(&url)
            .bind(&data)
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO channel_messages_embeds (message_id, embeds_id) VALUES (?, ?)")
            .bind(message_id)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(Embed { id, url, data })
    }
    pub async fn pin(message_id: Uuid, pinned: bool, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE channel_messages ch SET ch.pinned = ? WHERE ch.id = ?")
            .bind(pinned)
            .bind(message_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add_mention(
        message_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO messages_mentions (message_id, member_id) VALUES (?, ?)")
            .bind(message_id)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add_role_mention(
        message_id: Uuid,
        role_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO messages_role_mentions (message_id, role_id) VALUES (?, ?)")
            .bind(message_id)
            .bind(role_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_message_attachments(
        message_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Attachment>, Error> {
        Ok(sqlx::query_as(
            r#"
             SELECT
                 a.id,
                 a.filename,
                 a.url
             FROM
                 attachments a
                 INNER JOIN channel_messages_attachments cma ON a.id = cma.attachment_id
             WHERE
                 cma.message_id = ?
             "#,
        )
        .bind(message_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn add_attachment(
        message_id: Uuid,
        filename: &str,
        url: &str,
        pool: &MySqlPool,
    ) -> Result<Attachment, Error> {
        let id = Uuid::new_v4();
        sqlx::query(
            "
            INSERT INTO attachments
            (id, filename, url)
            VALUES (?, ?, ?)
        ",
        )
        .bind(id)
        .bind(filename)
        .bind(url)
        .execute(pool)
        .await?;
        sqlx::query(
            "INSERT INTO channel_messages_attachments (message_id, attachment_id) VALUES (?, ?)",
        )
        .bind(message_id)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(Attachment {
            id,
            filename: filename.to_string(),
            url: url.to_string(),
        })
    }

    pub async fn get_message_embeds(
        message_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Embed>, Error> {
        Ok(sqlx::query_as(
            r#"
                    SELECT
                        e.id,
                        e.url,
                        e.data
                    FROM
                        embeds e
                        INNER JOIN channel_messages_embeds cme ON e.id = cme.embeds_id
                    WHERE
                        cme.message_id = ?
                    "#,
        )
        .bind(message_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_message_mentions(
        message_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as(
            r#"
                    SELECT
                        m.*
                    FROM
                        members_with_profile_fallback m
                        INNER JOIN messages_mentions mm ON m.id = mm.member_id
                    WHERE
                        mm.message_id = ?
                    "#,
        )
        .bind(message_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_message_role_mentions(
        message_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Role>, Error> {
        Ok(sqlx::query_as(
            r#"
                    SELECT
                        r.id, r.name,r.server_id, r.can_edit, r.priority
                    FROM
                        roles r
                        INNER JOIN messages_role_mentions mrm ON r.id = mrm.role_id
                    WHERE
                        mrm.message_id = ?
                    "#,
        )
        .bind(message_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_message_reference(
        message_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<ChannelMessage, Error> {
        let sql_message: SqlChannelMessage = sqlx::query_as(
            r#"
            SELECT
                id,
                channel_id,
                sender_id,
                thread_id,
                message_reference,
                content,
                timestamp,
                edited_timestamp,
                pinned,
                mention_everyone
            FROM
                channel_messages
            WHERE
                id = ?
            "#,
        )
        .bind(message_id)
        .fetch_one(pool)
        .await?;

        let sender: Member =
            sqlx::query_as("SELECT * FROM members_with_profile_fallback WHERE id = ?")
                .bind(sql_message.sender_id)
                .fetch_one(pool)
                .await?;

        let mentions = ChannelMessage::get_message_mentions(sql_message.id, pool).await?;
        let mentions_roles =
            ChannelMessage::get_message_role_mentions(sql_message.id, pool).await?;
        let attachments = ChannelMessage::get_message_attachments(sql_message.id, pool).await?;
        let embeds = ChannelMessage::get_message_embeds(sql_message.id, pool).await?;

        Ok(ChannelMessage {
            id: sql_message.id,
            channel_id: sql_message.channel_id,
            thread_id: sql_message.thread_id,
            sender,
            message_reference: None,
            content: sql_message.content,
            timestamp: sql_message.timestamp,
            edited_timestamp: sql_message.edited_timestamp,
            pinned: sql_message.pinned,
            mention_everyone: sql_message.mention_everyone,
            mentions,
            mentions_roles,
            attachments,
            embeds,
            reactions: vec![],
        })
    }

    pub async fn get_thread_messages(
        thread_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<ChannelMessage>, Error> {
        let messages: Vec<SqlChannelMessage> = sqlx::query_as(
            r#"
            SELECT
                id as id,
                channel_id,
                thread_id,
                sender_id,
                message_reference as message_reference,
                content,
                timestamp,
                edited_timestamp,
                pinned,
                mention_everyone
            FROM
                channel_messages
            WHERE
                thread_id = ?
            ORDER BY
                timestamp ASC
            "#,
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?;

        let mut full_messages = vec![];

        for message in messages {
            let msg_sender: Member =
                sqlx::query_as("SELECT * FROM members_with_profile_fallback WHERE id = ?")
                    .bind(message.sender_id)
                    .fetch_one(pool)
                    .await?;

            let msg_reference = if let Some(reference) = message.message_reference {
                Some(Box::new(
                    ChannelMessage::get_message_reference(reference, pool).await?,
                ))
            } else {
                None
            };

            let msg_mentions = ChannelMessage::get_message_mentions(message.id, pool).await?;
            let msg_roles_mentions =
                ChannelMessage::get_message_role_mentions(message.id, pool).await?;
            let msg_attachments = ChannelMessage::get_message_attachments(message.id, pool).await?;
            let msg_embeds = ChannelMessage::get_message_embeds(message.id, pool).await?;
            let msg_reactions =
                ChannelMessage::get_message_reactions(message.id, member_id, pool).await?;
            full_messages.push(ChannelMessage {
                id: message.id,
                channel_id: message.channel_id,
                thread_id: message.thread_id,
                sender: msg_sender,
                message_reference: msg_reference,
                content: message.content,
                timestamp: message.timestamp,
                edited_timestamp: message.edited_timestamp,
                pinned: message.pinned,
                mention_everyone: message.mention_everyone,
                mentions: msg_mentions,
                mentions_roles: msg_roles_mentions,
                attachments: msg_attachments,
                embeds: msg_embeds,
                reactions: msg_reactions,
            });
        }

        Ok(full_messages)
    }

    pub async fn get_channel_messages(
        channel_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<ChannelMessage>, Error> {
        let messages: Vec<SqlChannelMessage> = sqlx::query_as(
            r#"
            SELECT
                id as id,
                channel_id,
                thread_id,
                sender_id,
                message_reference as message_reference,
                content,
                timestamp,
                edited_timestamp,
                pinned,
                mention_everyone
            FROM
                channel_messages
            WHERE
                channel_id = ?
            ORDER BY
                timestamp ASC
            "#,
        )
        .bind(channel_id)
        .fetch_all(pool)
        .await?;

        let mut full_messages = vec![];

        for message in messages {
            let msg_sender: Member =
                sqlx::query_as("SELECT * FROM members_with_profile_fallback WHERE id = ?")
                    .bind(message.sender_id)
                    .fetch_one(pool)
                    .await?;

            let msg_reference = if let Some(reference) = message.message_reference {
                let reference = ChannelMessage::get_message_reference(reference, pool).await;
                debug!("{reference:?}");
                Some(Box::new(reference?))
            } else {
                None
            };

            let msg_mentions = ChannelMessage::get_message_mentions(message.id, pool).await?;
            let msg_roles_mentions =
                ChannelMessage::get_message_role_mentions(message.id, pool).await?;
            let msg_attachments = ChannelMessage::get_message_attachments(message.id, pool).await?;
            let msg_embeds = ChannelMessage::get_message_embeds(message.id, pool).await?;
            let msg_reactions =
                ChannelMessage::get_message_reactions(message.id, member_id, pool).await?;
            full_messages.push(ChannelMessage {
                id: message.id,
                channel_id: message.channel_id,
                thread_id: message.thread_id,
                sender: msg_sender,
                message_reference: msg_reference,
                content: message.content,
                timestamp: message.timestamp,
                edited_timestamp: message.edited_timestamp,
                pinned: message.pinned,
                mention_everyone: message.mention_everyone,
                mentions: msg_mentions,
                mentions_roles: msg_roles_mentions,
                attachments: msg_attachments,
                embeds: msg_embeds,
                reactions: msg_reactions,
            });
        }

        Ok(full_messages)
    }

    async fn get_message_reactions(
        message_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Reaction>, Error> {
        let reactions = sqlx::query_as::<_, SqlReaction>(
            "
            SELECT re.* from reactions re where re.message_id = ?
        ",
        )
        .bind(message_id)
        .fetch_all(pool)
        .await;

        let mut full_reaction = vec![];
        for reaction in reactions? {
            let me = ChannelMessage::check_member_in_reaction(member_id, reaction.id, pool).await?;
            full_reaction.push(Reaction {
                id: reaction.id,
                message_id: reaction.message_id,
                name: String::from_utf8(reaction.name)?,
                counter: reaction.counter,
                me,
            });
        }
        Ok(full_reaction)
    }

    pub async fn add_channel_message(
        channel_id: Uuid,
        member_id: Uuid,
        message: &str,
        msg_reference: Option<Uuid>,
        pool: &MySqlPool,
    ) -> Result<ChannelMessage, Error> {
        let id = Uuid::new_v4();
        if let Some(reference) = msg_reference {
            sqlx::query(
                "
                INSERT INTO channel_messages
                (id, channel_id, sender_id, content, message_reference)
                VALUES (?, ?, ?, ?, ?)
            ",
            )
            .bind(id)
            .bind(channel_id)
            .bind(member_id)
            .bind(message)
            .bind(reference)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "
                INSERT INTO channel_messages
                (id, channel_id, sender_id, content)
                VALUES (?, ?, ?, ?)
            ",
            )
            .bind(id)
            .bind(channel_id)
            .bind(member_id)
            .bind(message)
            .execute(pool)
            .await?;
        }
        let sql_message: SqlChannelMessage = sqlx::query_as(
            r#"
            SELECT
                id,
                channel_id,
                thread_id,
                sender_id,
                message_reference,
                content,
                timestamp,
                edited_timestamp,
                pinned,
                mention_everyone
            FROM
                channel_messages
            WHERE
                id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        let sender: Member =
            sqlx::query_as("SELECT mv.* FROM members_with_profile_fallback mv WHERE mv.id = ?")
                .bind(sql_message.sender_id)
                .fetch_one(pool)
                .await?;
        let message_reference = if let Some(id) = msg_reference {
            Some(Box::new(
                ChannelMessage::get_message_reference(id, pool).await?,
            ))
        } else {
            None
        };
        Ok(ChannelMessage {
            id: sql_message.id,
            channel_id: sql_message.channel_id,
            thread_id: sql_message.thread_id,
            sender,
            message_reference,
            content: sql_message.content,
            timestamp: sql_message.timestamp,
            edited_timestamp: sql_message.edited_timestamp,
            pinned: sql_message.pinned,
            mention_everyone: sql_message.mention_everyone,
            mentions: vec![],
            mentions_roles: vec![],
            attachments: vec![],
            embeds: vec![],
            reactions: vec![],
        })
    }

    pub async fn get_pinned(
        channel_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<ChannelMessage>, Error> {
        let messages: Vec<SqlChannelMessage> = sqlx::query_as(
            r#"
            SELECT
                id as id,
                channel_id,
                thread_id,
                sender_id,
                message_reference as message_reference,
                content,
                timestamp,
                edited_timestamp,
                pinned,
                mention_everyone
            FROM
                channel_messages
            WHERE
                channel_id = ?
            AND
                pinned 
            ORDER BY
                timestamp ASC
            "#,
        )
        .bind(channel_id)
        .fetch_all(pool)
        .await?;

        let mut full_messages = vec![];

        for message in messages {
            let msg_sender: Member =
                sqlx::query_as("SELECT * FROM members_with_profile_fallback WHERE id = ?")
                    .bind(message.sender_id)
                    .fetch_one(pool)
                    .await?;

            let msg_reference = if let Some(reference) = message.message_reference {
                Some(Box::new(
                    ChannelMessage::get_message_reference(reference, pool).await?,
                ))
            } else {
                None
            };

            let msg_mentions = ChannelMessage::get_message_mentions(message.id, pool).await?;
            let msg_roles_mentions =
                ChannelMessage::get_message_role_mentions(message.id, pool).await?;
            let msg_attachments = ChannelMessage::get_message_attachments(message.id, pool).await?;
            let msg_embeds = ChannelMessage::get_message_embeds(message.id, pool).await?;
            full_messages.push(ChannelMessage {
                id: message.id,
                channel_id: message.channel_id,
                thread_id: message.thread_id,
                sender: msg_sender,
                message_reference: msg_reference,
                content: message.content,
                timestamp: message.timestamp,
                edited_timestamp: message.edited_timestamp,
                pinned: message.pinned,
                mention_everyone: message.mention_everyone,
                mentions: msg_mentions,
                mentions_roles: msg_roles_mentions,
                attachments: msg_attachments,
                embeds: msg_embeds,
                reactions: vec![],
            });
        }

        Ok(full_messages)
    }

    pub async fn inc_reaction_counter(reaction_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE reactions SET counter = counter + 1 WHERE id = ?")
            .bind(reaction_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn dec_reaction_counter(reaction_id: Uuid, pool: &MySqlPool) -> Result<u32, Error> {
        sqlx::query("UPDATE reactions SET counter = counter - 1 WHERE id = ?")
            .bind(reaction_id)
            .execute(pool)
            .await?;
        Ok(
            sqlx::query_as::<_, (u32,)>("SELECT counter FROM reactions WHERE id = ?")
                .bind(reaction_id)
                .fetch_one(pool)
                .await?
                .0,
        )
    }

    pub async fn delete_reaction(reaction_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM reactions WHERE id = ?")
            .bind(reaction_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add_member_to_reaction(
        reaction_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO reaction_members (reaction_id, member_id) VALUES (?, ?)")
            .bind(reaction_id)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn remove_member_to_reaction(
        reaction_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM reaction_members WHERE reaction_id = ? AND member_id = ?")
            .bind(reaction_id)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn check_member_in_reaction(
        member_id: Uuid,
        reaction_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<bool, Error> {
        Ok(sqlx::query_as::<_, (bool,)>(
            "
            SELECT EXISTS (
              SELECT 1
              FROM reaction_members
              WHERE reaction_id = ? AND member_id = ?
            )
        ",
        )
        .bind(reaction_id)
        .bind(member_id)
        .fetch_one(pool)
        .await?
        .0)
    }

    pub async fn create_reaction(
        message_id: Uuid,
        name: &str,
        pool: &MySqlPool,
    ) -> Result<Reaction, Error> {
        let reaction_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO reactions (id, message_id, name, counter) VALUES (?, ?, ?, ?)",
        )
        .bind(reaction_id)
        .bind(message_id)
        .bind(name)
        .bind(0)
        .execute(pool)
        .await;
        res?;
        Ok(Reaction {
            id: reaction_id,
            message_id,
            name: name.into(),
            counter: 0,
            me: false,
        })
    }

    pub async fn select_reaction(
        message_id: Uuid,
        member_id: Uuid,
        name: &str,
        pool: &MySqlPool,
    ) -> Result<Reaction, Error> {
        let reaction = sqlx::query_as::<_, SqlReaction>(
            "
                    SELECT re.* from reactions re where re.message_id = ? AND name = ?
                ",
        )
        .bind(message_id)
        .bind(name)
        .fetch_one(pool)
        .await?;

        let me = ChannelMessage::check_member_in_reaction(member_id, reaction.id, pool).await?;

        Ok(Reaction {
            id: reaction.id,
            message_id,
            name: String::from_utf8(reaction.name)?,
            counter: reaction.counter,
            me,
        })
    }
}
