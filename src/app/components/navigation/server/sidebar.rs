use super::category::Category;
use super::channel::Channel;
use super::server_menu::ServerMenu;
use crate::app::api::category::get_categories;
use crate::app::api::channel::get_general_channels;
use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use crate::entities::server::ServerStoreFields;
use leptos::prelude::*;
use reactive_stores::Store;
use uuid::Uuid;

use crate::entities::category::Category as EntCategory;
use crate::entities::channel::Channel as EntChannel;

#[derive(Clone)]
pub struct ServerSideBarContext {
    pub open: RwSignal<bool>,
}


//NOTE: create the store structs

#[component]
pub fn ServerSideBar() -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();

    let channels = Resource::new(move || server.id().get(), get_general_channels);

    let categories = Resource::new(move || server.id().get(), get_categories);
    let open = use_context::<ServerSideBarContext>()
        .expect("should acces teh server sidebar context")
        .open;
    view! {
        <div
            class="flex h-full relative inset-y-0 bg-base-300 z-40 ease-linear duration-200 transition-[width] overflow-hidden border-l-base-100 border-l border-0"
            style=move || if open.get() { "width: 240px" } else { "width: 0px" }
        >
            <div class="w-[240px] h-full flex flex-col items-center relative scrollbar-none overflow-y-scroll overflow-x-hidden shrink-0">
                <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                    <ServerMenu />
                    <div class="overflow-x-hidden overflow-y-scroll pr-2 flex-auto">
                        <Transition>
                            {move || {
                                Suspend::new(async move {
                                    let channels = channels.await;
                                    let categories = categories.await;
                                    match (channels, categories)  {
                                        (Ok(channels), Ok(categories)) => {
                                            let categories_store = Store::new(categories);
                                            let channel_store = Store::new(channels);
                                            view!{
                                                <For
                                                    each=move || channel_store.channels
                                                    key=|channel| channel.id
                                                    children=move |channel: EntChannel| {
                                                        view! { <Channel channel=channel.clone() /> }
                                                    }
                                                />
                                                <For
                                                    each=move || {
                                                        categories.get().and_then(Result::ok).unwrap_or_default()
                                                    }
                                                    key=|category| category.id
                                                    children=move |category: EntCategory| {
                                                        view! { <Category category=category.clone() /> }
                                                    }
                                                />

                                            }
                                        },
                                        => {}
                                    }
                                });
                            }}
                        </Transition>
                    </div>
                </div>
                <SideBarContextMenu server_id=server.id().get() />
            </div>
        </div>
    }
}

#[component]
pub fn ServerSideBarTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = use_context::<ServerSideBarContext>()
        .expect("shoul get the ServerSideBarContext context")
        .open;

    view! {
        <div class=class on:click=move |_| open.update(|open| *open = !*open)>
            {children.map(|children| children())}
        </div>
    }
}

#[component]
fn SideBarContextMenu(server_id: Uuid) -> impl IntoView {
    let hidden = RwSignal::new(false);
    let create_channel_node = NodeRef::new();
    let create_category_node = NodeRef::new();
    view! {
        <ContextMenuProvider modal=false hidden=hidden>
            <ContextMenuTrigger class="h-full w-full bg-none" />
            <ContextMenuContent
                ignore=vec![create_channel_node, create_category_node]
                class="z-40 select-none"
            >
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <CreateChannelModal
                        content_ref=create_channel_node
                        server_id=server_id
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                        on_click=Signal::derive(move || hidden.set(false))
                    >
                        <div>"Create Channel"</div>
                    </CreateChannelModal>
                    <CreateCategoryModal
                        content_ref=create_category_node
                        server_id=server_id
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                        on_click=Signal::derive(move || hidden.set(false))
                    >
                        <div>"Create Category"</div>
                    </CreateCategoryModal>
                </div>
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}
