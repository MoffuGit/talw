use crate::entities::user::User;
use leptos::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use bcrypt::verify;
        use crate::entities::user::AuthSession;
        use sqlx::MySqlPool;

        pub fn pool() -> Result<MySqlPool, ServerFnError> {
            use_context::<MySqlPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
        }

        pub fn auth_user() -> Result<User, ServerFnError> {
            auth()?.current_user.ok_or_else(|| ServerFnError::ServerError("cant auth user".to_string()))
        }
    }
}

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub auth: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub signup: Action<Signup, Result<(), ServerFnError>>,
}

pub fn provide_auth_context() {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let auth = create_resource(
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup.version().get(),
            )
        },
        move |_| get_user(),
    );

    provide_context(AuthContext {
        auth,
        login,
        logout,
        signup,
    });
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("have auth context")
}

pub fn current_user() -> Signal<Option<Result<Option<User>, ServerFnError>>> {
    Signal::derive(move || use_auth().auth.get())
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;
    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let user = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("User dont exist".into()))?;

    match verify(password, &user.password) {
        Ok(true) => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        Ok(false) => Err(ServerFnError::ServerError("Password dont match".into())),
        _ => Err(ServerFnError::ServerError(
            "Cannot verify your password".into(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    confirmation_password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    if password != confirmation_password {
        return Err(ServerFnError::ServerError("Password did not match".into()));
    }

    User::create(username.clone(), password, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("Cant create user".into()))?;
    let user = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("Signup failed".into()))?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());
    leptos_axum::redirect("/");
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");
    Ok(())
}
