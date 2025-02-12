pub mod category;
pub mod channel;
pub mod member;
pub mod role;
pub mod server;
pub mod thread;
pub mod user;

#[derive(Debug)]
#[cfg(feature = "ssr")]
pub enum Error {
    NotFound,
    Sqlx(sqlx::Error),
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            err => Error::Sqlx(err),
        }
    }
}

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
impl From<Error> for ServerFnError {
    fn from(_: Error) -> Self {
        ServerFnError::new("Something go wrong in our servers")
    }
}
