use crate::entities::member::{Member, Status};
use crate::entities::role::Role;
use crate::entities::user::Profile;
use cfg_if::cfg_if;
use leptos::prelude::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::auth_user;
        use super::user_can_edit;
        use super::pool;
        use crate::entities::user::User;
    }
}

#[server(GetMemberNameAndUrl)]
pub async fn get_member_profile(member_id: Uuid) -> Result<Option<Profile>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    let member = Member::get(member_id, &pool).await?;

    Ok(User::get_profile(member.user_id, &pool).await.ok())
}

#[server(GetMember)]
pub async fn get_member(server_id: Uuid) -> Result<Member, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Ok(Member::get_from_user_on_server(user.id, server_id, &pool).await?)
}

#[server(GetUnfilterThreadMembers)]
pub async fn get_unfilter_thread_members(thread_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Member::get_unfilter_thread_members(thread_id, &pool).await?)
}

#[server(GetThreadMembers)]
pub async fn get_thread_members(thread_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Member::get_thread_members(thread_id, &pool).await?)
}

#[server(GetMembers)]
pub async fn get_members(server_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Member::get_members(server_id, &pool).await?)
}

#[server(GetMemberRoles)]
pub async fn get_member_roles(member_id: Uuid) -> Result<Vec<Role>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Role::get_member_roles(member_id, &pool).await?)
}

#[server(MemberCanEdit)]
pub async fn member_can_edit(server_id: Uuid) -> Result<bool, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    user_can_edit(server_id, user.id, &pool).await
}
