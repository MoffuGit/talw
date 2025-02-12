use crate::entities::category::Category;
use cfg_if::cfg_if;
use leptos::prelude::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::entities::server::Server;
        use crate::entities::channel::Channel;
        use super::user_can_edit;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct CategoryContext {
    pub create_category: ServerAction<CreateCategory>,
    pub rename_category: ServerAction<RenameCategory>,
    pub delete_category: ServerAction<DeleteCategory>,
}

pub fn provide_category_context() {
    let create_category = ServerAction::<CreateCategory>::new();
    let rename_category = ServerAction::<RenameCategory>::new();
    let delete_category = ServerAction::<DeleteCategory>::new();
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

    Ok(Server::get_server_categories(server_id, &pool).await?)
}

#[server(CreateCategory, "/api")]
pub async fn create_category(server_id: Uuid, name: String) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if name.len() <= 1 {
            return Err(ServerFnError::new("min len is 1"));
        };
        return Ok(Category::create(name, server_id, &pool).await?);
    }
    Err(ServerFnError::new("You can't create the category"))
}

#[server(RenameCategory)]
pub async fn rename_category(
    server_id: Uuid,
    category_id: Uuid,
    new_name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if new_name.len() <= 1 {
            return Err(ServerFnError::new("min len is 1"));
        }
        return Ok(Category::rename(new_name, category_id, server_id, &pool).await?);
    };

    Err(ServerFnError::new("You can't rename the category"))
}

#[server(DeleteCategory)]
pub async fn delete_category(server_id: Uuid, category_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        Channel::remove_all_from_category(server_id, category_id, &pool).await?;
        Category::delete(category_id, server_id, &pool).await?;
        return Ok(());
    };
    Err(ServerFnError::new("You can't delete the category"))
}
