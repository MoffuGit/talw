use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use async_trait::async_trait;
        use axum_session::SessionMySqlPool;
        use axum_session_auth::Authentication;
        use bcrypt::{hash, DEFAULT_COST};
        use sqlx::{FromRow, MySqlPool};
        pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionMySqlPool, MySqlPool>;
        use super::Error;

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Banner {
    pub id: Uuid,
    pub image_url: Option<String>,
    pub primary_color: Option<String>,
    pub accent_color: Option<String>,
    pub about: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn get_banner(user_id: Uuid, pool: &MySqlPool) -> Result<Banner, Error> {
        Ok(
            sqlx::query_as::<_, Banner>("SELECT * FROM banners WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_profile(user_id: Uuid, pool: &MySqlPool) -> Result<Profile, Error> {
        Ok(
            sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get(id: Uuid, pool: &MySqlPool) -> Result<User, Error> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_from_name(name: String, pool: &MySqlPool) -> Result<User, Error> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?")
                .bind(name)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn create(name: String, password: String, pool: &MySqlPool) -> Result<Uuid, Error> {
        let password = hash(password, DEFAULT_COST).unwrap();
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO users (id, name, password) VALUES (?,?,?)")
            .bind(id)
            .bind(name)
            .bind(password)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn create_profile(
        name: String,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO profiles (id, name, user_id) VALUES (?,?,?)")
            .bind(id)
            .bind(name)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn create_banner(user_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO banners (id,user_id) VALUES (?,?)")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_banner_image_key(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Option<String>, Error> {
        Ok(sqlx::query_as::<_, (Option<String>,)>(
            "SELECT banners.image_key FROM banners WHERE banners.user_id = ?",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?
        .0)
    }

    pub async fn get_profile_image_key(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Option<String>, Error> {
        Ok(sqlx::query_as::<_, (Option<String>,)>(
            "SELECT profiles.image_key FROM profiles WHERE profiles.user_id = ?",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?
        .0)
    }

    pub async fn set_image_banner_url(
        url: String,
        key: String,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query(
            "UPDATE banners SET banners.image_url = ?, banners.image_key = ? WHERE banners.user_id = ?",
        )
        .bind(url)
        .bind(key)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn set_image_profile_url(
        url: String,
        key: String,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        let res = sqlx::query(
            "UPDATE profiles SET profiles.image_url = ?, profiles.image_key = ? WHERE profiles.user_id = ?",
        )
        .bind(url)
        .bind(key)
        .bind(user_id)
        .execute(pool)
        .await;
        println!("{res:?}");
        Ok(())
    }

    pub async fn set_profile_name(
        user_id: Uuid,
        new_name: String,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE profiles SET profiles.name = ? WHERE profiles.user_id = ?")
            .bind(new_name)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn set_banner_about(
        user_id: Uuid,
        new_about: String,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE banners SET banners.about = ? WHERE banners.user_id = ?")
            .bind(new_about)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
#[cfg(feature = "ssr")]
impl Authentication<User, Uuid, MySqlPool> for User {
    async fn load_user(userid: Uuid, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get(userid, pool)
            .await
            .or(Err(anyhow::anyhow!("Cannot get user")))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
