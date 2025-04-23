use super::user::get_user;
use crate::entities::user::User;
use cfg_if::cfg_if;
use leptos::prelude::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use futures::join;
        use bcrypt::verify;
        use super::pool;
        use super::auth;
    }
}

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub auth: Resource<Result<Option<User>, ServerFnError>>,
    pub login: ServerAction<Login>,
    pub logout: ServerAction<Logout>,
    pub signup: ServerAction<Signup>,
}

pub fn provide_auth_context() {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();

    let auth = Resource::new(
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

#[server(Login)]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let user = match User::get_from_name(username, &pool).await {
        Ok(user) => user,
        Err(crate::entities::Error::NotFound) => {
            return Err(ServerFnError::new("This user don't exist"))
        }
        Err(_) => {
            return Err(ServerFnError::new(
                "Looks like we are having problems on our servers",
            ))
        }
    };

    let real_password = User::get_password(user.id, &pool).await?;

    match verify(&password, &real_password) {
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

    let user_id = User::create(username.clone(), password, &pool).await?;

    let (profile, banner) = join!(
        User::create_profile(username.clone(), user_id, &pool),
        User::create_banner(user_id, &pool)
    );
    profile?;
    banner?;

    auth.login_user(user_id);
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
