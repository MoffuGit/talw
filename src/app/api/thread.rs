use uuid::Uuid;

use crate::entities::member::Member;
use crate::entities::thread::Thread;
use crate::messages::Message;
use cfg_if::cfg_if;
use core::f64;
use leptos::prelude::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::msg_sender;
        use super::user_can_edit;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ThreadContext {
    pub create_thread: ServerAction<CreateThread>,
    pub join_thread: ServerAction<JoinThread>,
    pub leave_thread: ServerAction<LeaveThread>,
    pub delete_thread: ServerAction<DeleteThread>,
}

pub fn use_thread() -> ThreadContext {
    use_context::<ThreadContext>().expect("have thread context")
}

pub fn provide_thread_context() {
    let join_thread = ServerAction::<JoinThread>::new();
    let leave_thread = ServerAction::<LeaveThread>::new();
    let create_thread = ServerAction::<CreateThread>::new();
    let delete_thread = ServerAction::<DeleteThread>::new();

    provide_context(ThreadContext {
        create_thread,
        join_thread,
        leave_thread,
        delete_thread,
    })
}

#[server(GetThreadMembersWithoutRole)]
pub async fn get_thread_members_without_role(
    thread_id: Uuid,
) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Thread::get_members_witout_role(thread_id, &pool).await?)
}

#[server(GetThreadMembersWithRole)]
pub async fn get_thread_members_with_role(
    role_id: Uuid,
    thread_id: Uuid,
) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Thread::get_members_with_role(role_id, thread_id, &pool).await?)
}

#[server(GetThreadsForMember)]
pub async fn get_threads_for_member(
    channel_id: Uuid,
    member_id: Uuid,
) -> Result<Vec<Thread>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Thread::get_threads_for_member(channel_id, member_id, &pool).await?)
}

#[server(CheckMemberOnThread)]
pub async fn check_member_on_thread(thread_id: Uuid) -> Result<bool, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Ok(Thread::check_member(thread_id, user.id, &pool).await?)
}

#[server(CreateThread)]
pub async fn create_thread(
    channel_id: Uuid,
    server_id: Uuid,
    name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if let Ok(member) = Member::get_user_member(user.id, server_id, &pool).await {
        let id = Thread::create(name, channel_id, member.id, &pool).await?;
        Thread::add_member(id, member.id, &pool).await?;
        leptos_axum::redirect(&format!(
            "/servers/{}/{}/{}",
            server_id.simple(),
            channel_id.simple(),
            id.simple()
        ));
        msg_sender()?.send(Message::ThreadCreated {
            server_id,
            thread_id: id,
        });
        Ok(())
    } else {
        Err(ServerFnError::new("You can't create a thread"))
    }
}

#[server(JoinThread)]
pub async fn join_thread(thread_id: Uuid, server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    if let Ok(member) = Member::get_user_member(user.id, server_id, &pool).await {
        Thread::add_member(thread_id, member.id, &pool).await?;
        Ok(())
    } else {
        Err(ServerFnError::new("You join into this thread"))
    }
}

#[server(LeaveThread)]
pub async fn leave_thread(thread_id: Uuid, server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    if let Ok(member) = Member::get_user_member(user.id, server_id, &pool).await {
        Thread::remove_member(thread_id, member.id, &pool).await?;
        Ok(())
    } else {
        Err(ServerFnError::new("You can't leave this thread"))
    }
}

#[server(DeleteThread)]
pub async fn delete_thread(thread_id: Uuid, server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        Thread::delete_members(thread_id, &pool).await?;
        Thread::delete(thread_id, &pool).await?;
        msg_sender()?.send(Message::ThreadDeleted {
            server_id,
            thread_id,
        });
        return Ok(());
    }
    if let Ok(member) = Member::get_user_member(user.id, server_id, &pool).await {
        if Thread::get_created_by(thread_id, &pool).await? == member.id {
            Thread::delete_members(thread_id, &pool).await?;
            Thread::delete(thread_id, &pool).await?;
            msg_sender()?.send(Message::ThreadDeleted {
                server_id,
                thread_id,
            });
            return Ok(());
        }
    }
    Err(ServerFnError::new("You can't delte this"))
}

#[server(GetThread)]
pub async fn get_thread(thread_id: Uuid, channel_id: Uuid) -> Result<Thread, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Thread::get(thread_id, channel_id, &pool).await?)
}

#[server(GetThreadsFromChannel)]
pub async fn get_threads_from_channel(channel_id: Uuid) -> Result<Vec<Thread>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Thread::get_threads_from_channel(channel_id, &pool).await?)
}

#[server(ToggleThreadWidth, "/api")]
pub async fn toggle_thread_width(width: f64) -> Result<f64, ServerFnError> {
    use tower_cookies::cookie::SameSite;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    let cookies = use_context::<Cookies>().expect("Have tower_cookies::Cookies provided");

    cookies.add(
        Cookie::build(("thread_width", width.to_string()))
            .path("/")
            .same_site(SameSite::Strict)
            .into(),
    );

    Ok(width)
}
cfg_if! {
    if  #[cfg(not(feature = "ssr"))] {
        pub fn initial_width() -> f64 {
            use wasm_bindgen::JsCast;

            let doc = document().unchecked_into::<web_sys::HtmlDocument>();
            let cookie = doc.cookie().unwrap_or_default();
            cookie
                .split(';')
                .find(|cookie| cookie.contains("thread_width"))
                .and_then(|cookie| {
                    cookie
                        .split('=')
                        .last()
                        .and_then(|width| width.parse::<f64>().ok())
                })
                .unwrap_or(400.0)
        }
    } else {
        pub fn initial_width() -> f64 {
            use tower_cookies::Cookies;

            use_context::<Cookies>()
                .map(|cookies| {
                    cookies
                        .get("thread_width")
                        .and_then(|width| width.value().parse::<f64>().ok()).unwrap_or(400.0)
                })
            .unwrap_or(400.0)
        }
    }
}
