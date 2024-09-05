use crate::entities::user::User;
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use bcrypt::verify;
        use super::pool;
        use super::auth;
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

    let user = match User::get_from_username(username, &pool).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return Err(ServerFnError::new("This user don't exist")),
        Err(_) => {
            return Err(ServerFnError::new(
                "Looks like we are having problems on our servers",
            ))
        }
    };

    match verify(password, &user.password) {
        Ok(true) => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        Ok(false) => Err(ServerFnError::new("Your passwords dont match")),
        _ => Err(ServerFnError::new(
            "We cant verify your password".to_string(),
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

    if username.len() < 4 || username.len() > 20 {
        return Err(ServerFnError::new(
            "Your username needs to be between 4 and 20 character long",
        ));
    }

    if password.len() < 8 || password.len() > 20 {
        return Err(ServerFnError::new(
            "Your password needs to be between 8 and 20 character long",
        ));
    }

    if password != confirmation_password {
        return Err(ServerFnError::new("Your password dont match"));
    }

    let user = User::create(username.clone(), password, &pool)
        .await
        .map(|id| async move {
            User::get(id, &pool).await.or(Err(ServerFnError::new(
                "Something go wrong creating your account".to_string(),
            )))
        })
        .or(Err(ServerFnError::new(
            "We can't create your account in this moments".to_string(),
        )))?
        .await?;

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
