use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::member::Role;
use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::auth_member;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct CategoryContext {
    pub create_category: Action<CreateCategory, Result<Uuid, ServerFnError>>,
    pub rename_category: Action<RenameCategory, Result<(), ServerFnError>>,
    pub delete_category: Action<DeleteCategory, Result<(), ServerFnError>>,
}

pub fn provide_category_context() {
    let create_category = create_server_action::<CreateCategory>();
    let rename_category = create_server_action::<RenameCategory>();
    let delete_category = create_server_action::<DeleteCategory>();
    provide_context(CategoryContext {
        create_category,
        rename_category,
        delete_category,
    })
}

pub fn use_category() -> CategoryContext {
    use_context::<CategoryContext>().expect("have category context")
}

#[server(GetCategories, "/api")]
pub async fn get_categories(server_id: Uuid) -> Result<Vec<Category>, ServerFnError> {
    let _ = auth_user()?;
    let pool = pool()?;

    let categories = Server::get_categories(server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant get the categories".to_string()))?;

    Ok(categories)
}

#[server(CreateCategory, "/api")]
pub async fn create_category(server_id: Uuid, name: String) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;

    if name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    Category::create(name, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant create the category"))
}

#[server(RenameCategory)]
pub async fn rename_category(
    server_id: Uuid,
    category_id: Uuid,
    new_name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;

    if new_name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    Category::rename(new_name, category_id, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant change the name"))
}

#[server(DeleteCategory)]
pub async fn delete_category(server_id: Uuid, category_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    Channel::remove_all_from_category(server_id, category_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant delte the channel"))?;
    Category::delete(category_id, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant delte the channel"))
}
