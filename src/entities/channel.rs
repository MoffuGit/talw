use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum Type {
    TEXT,
    GUEST,
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::MySql> for Type {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: Type,
    pub server_id: Uuid,
}

impl Channel {
    //NOTE:
    //agregar la funcionm para agregar un canal a un servidor, luego, cada vez que se crean
    //servidores crear por defecto dos canales
}
