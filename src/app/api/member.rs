use crate::entities::member::AboutMember;
use crate::entities::member::Member;
use crate::entities::role::Role;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::entities::user::User;
        use super::auth_user;
        use super::user_can_edit;
        use super::pool;

    }
}

#[server(GetMemberNameAndUrl)]
pub async fn get_member_name_and_url(
    member_id: Uuid,
) -> Result<(String, Option<String>), ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Member::get_name_and_url(member_id, &pool).await?)
}

#[server(GetMember)]
pub async fn get_member(server_id: Uuid) -> Result<Member, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Ok(Member::get_member(user.id, server_id, &pool).await?)
}

#[server(GetThreadMembers)]
pub async fn get_thread_members(thread_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Member::get_thread_members(thread_id, &pool).await?)
}

#[server(GetMemberAbout)]
pub async fn get_member_about(member_id: Uuid) -> Result<AboutMember, ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    Ok(Member::get_about(member_id, &pool).await?)
}

#[server(GetMemberRoles)]
pub async fn get_member_roles(member_id: Uuid) -> Result<Vec<Role>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Role::get_member_roles(member_id, &pool).await?)
}

#[server(GetUserNameFromMember)]
pub async fn get_user_name_from_member(member_id: Uuid) -> Result<String, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_user_name_from_member(member_id, &pool).await?)
}

#[server(GetMembersWithoutRole)]
pub async fn get_members_without_role(server_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    Ok(Member::get_members_without_role(server_id, &pool).await?)
}

#[server(GetMembersFromRole)]
pub async fn get_members_from_role(role_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    Ok(Member::get_member_from_role(role_id, &pool).await?)
}

#[server(GetUserMembers, "/api")]
pub async fn get_user_members() -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Ok(Member::get_user_members(user.id, &pool).await?)
}

#[server(MemberCanEdit)]
pub async fn member_can_edit(server_id: Uuid) -> Result<bool, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    user_can_edit(server_id, user.id, &pool).await
}
