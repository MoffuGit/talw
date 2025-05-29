use cfg_if::cfg_if;
use log::debug;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Decode, Encode};
        use sqlx::QueryBuilder;
        use super::Error;
        use super::server::Server;
        use super::role::Role;
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Store, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Member {
    pub id: Uuid,
    pub user_id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub status: Status,
    pub role_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Eq)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum Status {
    ONLINE,
    OFFLINE,
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::MySql> for Status {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn get_server_members(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT mv.*
            FROM members_with_profile_fallback mv 
            WHERE mv.server_id = ?
        "#,
        )
        .bind(server_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn update_member_status(
        member_id: Uuid,
        status: Status,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE members SET members.status = ? WHERE members.id = ?")
            .bind(status)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_members_status(
        user_id: Uuid,
        status: Status,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE members SET members.status = ? WHERE members.user_id = ?")
            .bind(status)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get(member_id: Uuid, pool: &MySqlPool) -> Result<Member, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                SELECT mv.* 
                FROM members_with_profile_fallback mv 
                WHERE mv.id = ?
            "#,
        )
        .bind(member_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn check_member_on_server(
        member_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                SELECT mv.* 
                FROM members_with_profile_fallback mv 
                WHERE mv.id = ? 
                AND mv.server_id = ?
            "#,
        )
        .bind(member_id)
        .bind(server_id)
        .fetch_one(pool)
        .await?)
    }
    pub async fn get_from_user_on_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        let result = sqlx::query_as::<_, Member>(
            r#"
                SELECT *
                FROM members_with_profile_fallback mv 
                WHERE mv.user_id = ? 
                AND mv.server_id = ?
            "#,
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_one(pool)
        .await;
        debug!("{result:?}");
        Ok(result?)
    }

    pub async fn get_thread_members(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            "SELECT mv.*
            FROM members_with_profile_fallback mv 
            JOIN threads_members 
            ON mv.id = threads_members.member_id 
            JOIN threads 
            ON threads.id = threads_members.thread_id 
            LEFT JOIN roles r ON mv.role_id = r.id
            WHERE threads.id = ?
            ORDER BY
                CASE
                    WHEN mv.status = 'ONLINE' THEN 0
                    WHEN mv.status = 'OFFLINE' THEN 1
                END ASC,
                CASE
                    WHEN mv.role_id IS NOT NULL THEN r.priority
                    ELSE 0
                END DESC
        ",
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_thread_filtered_members(
        thread_id: Uuid,
        role_id: Option<Uuid>,
        status: Option<Status>,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        let mut query_builder = QueryBuilder::new(
            "
            SELECT mv.*
            FROM members_with_profile_fallback mv 
            JOIN threads_members 
            ON mv.id = threads_members.member_id 
            JOIN threads 
            ON threads.id = threads_members.thread_id 
            LEFT JOIN roles r ON mv.role_id = r.id
            WHERE threads.id = ",
        );
        query_builder.push_bind(thread_id);
        if role_id.is_some() {
            query_builder.push(" AND mv.role_id = ").push_bind(role_id);
        }
        if status.is_some() {
            query_builder.push(" AND mv.status = ").push_bind(status);
        }
        query_builder.push(
            r#"
        ORDER BY
            CASE
                WHEN mv.status = 'ONLINE' THEN 0
                WHEN mv.status = 'OFFLINE' THEN 1
                ELSE 2
            END ASC,
            CASE
                WHEN mv.role_id IS NOT NULL THEN r.priority
                ELSE 0
            END DESC
        "#,
        );
        let query = query_builder.build_query_as::<Member>();

        Ok(query.fetch_all(pool).await?)
    }

    pub async fn member_can_edit(user: Uuid, pool: &MySqlPool) -> Result<bool, Error> {
        match Role::get_member_roles(user, pool)
            .await?
            .iter()
            .find(|role| role.can_edit)
        {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn get_user_members(user_id: Uuid, pool: &MySqlPool) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
        SELECT mv.*
        FROM members_with_profile_fallback mv 
        WHERE mv.user_id = ?"#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_user_member(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        let res = sqlx::query_as::<_, Member>(
            "SELECT mv.* FROM members_with_profile_fallback mv WHERE mv.user_id = ? AND mv.server_id = ?",
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_one(pool)
        .await;
        Ok(res?)
    }
    pub async fn create(
        user: Uuid,
        server: Uuid,
        name: &str,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO members (id, user_id, server_id,name) VALUES (?,?,?, ?)")
            .bind(id)
            .bind(user)
            .bind(server)
            .bind(name)
            .execute(pool)
            .await?;
        Ok(Member {
            id,
            user_id: user,
            server_id: server,
            name: name.to_string(),
            image_url: None,
            status: Status::OFFLINE,
            role_id: None,
        })
    }

    pub async fn create_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        name: &str,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        let server_id = Server::get_from_invitation(invitation, pool).await?;
        sqlx::query("INSERT INTO members (id, user_id, server_id, name) VALUES(?, ?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(server_id)
            .bind(name)
            .execute(pool)
            .await?;
        Ok(server_id)
    }

    pub async fn check_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        Ok(sqlx::query_as::<_, (Uuid,)>("SELECT servers.id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await?.0)
    }

    pub async fn delete_from_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM members WHERE user_id=? AND server_id=?")
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_members(server_id: Uuid, pool: &MySqlPool) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT mv.*
            FROM members_with_profile_fallback mv
            LEFT JOIN roles r ON mv.role_id = r.id
            WHERE mv.server_id = ?
            ORDER BY
                CASE
                    WHEN mv.status = 'ONLINE' THEN 0
                    WHEN mv.status = 'OFFLINE' THEN 1
                END ASC,
                CASE
                    WHEN mv.role_id IS NOT NULL THEN r.priority
                    ELSE 0
                END DESC
            "#,
        )
        .bind(server_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_filtred_members(
        server_id: Uuid,
        role_id: Option<Uuid>,
        status: Option<Status>,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        let mut query_builder = QueryBuilder::new(
            r#"
        SELECT mv.*
        FROM members_with_profile_fallback mv
        LEFT JOIN roles r ON mv.role_id = r.id
        WHERE mv.server_id = "#,
        );

        query_builder.push_bind(server_id);

        if role_id.is_some() {
            query_builder.push(" AND mv.role_id = ").push_bind(role_id);
        }

        if status.is_some() {
            query_builder.push(" AND mv.status = ").push_bind(status);
        }

        query_builder.push(
            r#"
        ORDER BY
            CASE
                WHEN mv.status = 'ONLINE' THEN 0
                WHEN mv.status = 'OFFLINE' THEN 1
                ELSE 2
            END ASC,
            CASE
                WHEN mv.role_id IS NOT NULL THEN r.priority
                ELSE 0
            END DESC
        "#,
        );

        let query = query_builder.build_query_as::<Member>();

        Ok(query.fetch_all(pool).await?)
    }

    pub async fn get_offline_members(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                SELECT mv.*
                FROM members_with_profile_fallback mv
                WHERE mv.server_id = ?
                AND mv.status = 'OFFLINE'
            "#,
        )
        .bind(server_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_thread_online_members_without_role(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT mv.*
            FROM members_with_profile_fallback mv
            JOIN threads_members tm ON mv.id = tm.member_id
            WHERE tm.thread_id = ?
            AND NOT mv.status = 'OFFLINE'
            AND NOT EXISTS (
                SELECT 1
                FROM member_roles mr
                WHERE mr.member_id = mv.id
            )
            "#,
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_thread_offline_members(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT mv.*
            FROM members_with_profile_fallback mv
            JOIN threads_members tm ON mv.id = tm.member_id
            WHERE tm.thread_id = ?
            AND mv.status = 'OFFLINE'
            "#,
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_thread_online_members_with_role(
        thread_id: Uuid,
        role_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                SELECT mv.*
                FROM members_with_profile_fallback mv
                JOIN threads_members tm ON mv.id = tm.member_id
                JOIN member_roles mr ON mv.id = mr.member_id
                JOIN roles r ON mr.role_id = r.id
                WHERE tm.thread_id = ?
                AND NOT mv.status = 'OFFLINE'
                AND r.id = ? -- Check if the role ID matches
                AND NOT EXISTS (
                    SELECT 1
                    FROM member_roles mr2
                    JOIN roles r2 ON mr2.role_id = r2.id
                    WHERE mr2.member_id = mv.id
                    AND r2.priority > r.priority
                )
            "#,
        )
        .bind(thread_id)
        .bind(role_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_members_with_role(
        role_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT DISTINCT mv.*
            FROM members_with_profile_fallback mv
            WHERE mv.role_id = ?
            "#,
        )
        .bind(role_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_five_thread_members(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            "SELECT mv.*
            FROM members_with_profile_fallback mv 
            JOIN threads_members 
            ON mv.id = threads_members.member_id 
            JOIN threads 
            ON threads.id = threads_members.thread_id 
            WHERE mv.server_id = ?
            LIMIT 5
        ",
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?)
    }
}
