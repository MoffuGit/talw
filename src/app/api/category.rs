use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::member::{Member, Role};
use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
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
    auth_user()?;
    let pool = pool()?;

    Server::get_server_categories(server_id, &pool)
        .await
        .or(Err(ServerFnError::new("We can't find the categories")))
}

#[server(CreateCategory, "/api")]
pub async fn create_category(server_id: Uuid, name: String) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }
    match Member::get_member_role(server_id, user.id, &pool).await {
        Ok(Role::ADMIN) => Category::create(name, server_id, &pool)
            .await
            .or(Err(ServerFnError::new("We cant create the category"))),
        Ok(_) | Err(sqlx::Error::RowNotFound) => {
            Err(ServerFnError::new("You can't create the category"))
        }
        Err(_) => Err(ServerFnError::new("Something go wrong in our servers")),
    }
}

#[server(RenameCategory)]
pub async fn rename_category(
    server_id: Uuid,
    category_id: Uuid,
    new_name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if new_name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    match Member::get_member_role(server_id, user.id, &pool).await {
        Ok(Role::ADMIN) => Category::rename(new_name, category_id, server_id, &pool)
            .await
            .or(Err(ServerFnError::new("We cant change the name"))),
        Ok(_) | Err(sqlx::Error::RowNotFound) => {
            Err(ServerFnError::new("You can't rename the category"))
        }
        Err(_) => Err(ServerFnError::new("Something go wrong in our servers")),
    }
}

#[server(DeleteCategory)]
pub async fn delete_category(server_id: Uuid, category_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    match Member::get_member_role(server_id, user.id, &pool).await {
        Ok(Role::ADMIN) => {
            Channel::remove_all_from_category(server_id, category_id, &pool)
                .await
                .or(Err(ServerFnError::new("cant delete the channel")))?;
            Category::delete(category_id, server_id, &pool)
                .await
                .or(Err(ServerFnError::new("cant delte the channel")))
        }
        Ok(_) | Err(sqlx::Error::RowNotFound) => {
            Err(ServerFnError::new("You can't delete the category"))
        }
        Err(_) => Err(ServerFnError::new("Something go wrong in our servers")),
    }
}
