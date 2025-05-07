use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::member::Member;
use super::role::Role;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
        use super::Error;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ChannelMessage {
    id: Uuid,
    channel_id: Uuid,
    sender: Member,
    message_reference: Option<Box<ChannelMessage>>,
    content: String,
    timestamp: DateTime<Utc>,
    edited_timestamp: Option<DateTime<Utc>>,
    pinned: bool,
    mention_everyone: bool,
    mentions: Vec<Member>,
    mentions_roles: Vec<Role>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    //reactions: Vec<Reaction>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Attachment {
    id: Uuid,
    filename: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Embed {
    id: Uuid,
    url: String,
    data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct SqlChannelMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
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
                        m.id,
                        m.username,
                        m.avatar_url
                    FROM
                        members m
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
                        r.id,
                        r.name,
                        r.permissions
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

        let sender: Member = sqlx::query_as("SELECT * FROM members WHERE id = ?")
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
        })
    }

    pub async fn get_channel_messages(
        channel_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<ChannelMessage>, Error> {
        let messages: Vec<SqlChannelMessage> = sqlx::query_as(
            r#"
            SELECT
                id as id,
                channel_id,
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
            let msg_sender: Member = sqlx::query_as("SELECT * FROM members WHERE id = ?")
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
            });
        }

        Ok(full_messages)
    }
}
