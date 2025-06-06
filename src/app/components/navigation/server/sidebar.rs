use std::collections::HashMap;

use super::category::Category;
use super::channel::Channel;
use super::server_menu::ServerMenu;
use crate::app::api::category::get_categories;
use crate::app::api::channel::get_channels;
use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use crate::app::stores::CategoryStoreSync;
use crate::app::stores::ChannelStoreSync;
use crate::app::sync::use_sync;
use crate::entities::category::Category as CategoryEntitie;
use crate::entities::category::CategoryStoreFields;
use crate::entities::channel::Channel as ChannelStruct;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use leptos::prelude::*;
use log::debug;
use reactive_stores::Store;
use uuid::Uuid;

#[derive(Clone)]
pub struct ServerSideBarContext {
    pub open: RwSignal<bool>,
}

#[derive(Debug, Clone, Store)]
pub struct ChannelStore {
    #[store(key: Uuid = |channel| channel.id)]
    channels: Vec<ChannelStruct>,
}

#[derive(Debug, Clone, Store)]
pub struct CategoryStore {
    #[store(key: Uuid = |category| category.id)]
    categories: Vec<CategoryEntitie>,
}

#[component]
pub fn ServerSideBar() -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();

    let channels = Resource::new(move || server.id().get(), get_channels);

    let categories = Resource::new(move || server.id().get(), get_categories);
    let open = use_context::<ServerSideBarContext>()
        .expect("should acces teh server sidebar context")
        .open;
    view! {
        <div
            class="flex h-full relative inset-y-0 bg-base-300 z-40 ease-linear duration-200 transition-[width] overflow-hidden border-l-base-100 border-l border-0 shrink-0"
            style=move || if open.get() { "width: 240px" } else { "width: 0px" }
        >
            <div class="w-[240px] h-full flex flex-col items-center relative shrink-0">
                <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                    <ServerMenu />
                    <div class="scrollbar-none overflow-y-scroll overflow-x-hidden flex flex-col relative w-full items-stretch">
                        <Transition>
                            {move || Suspend::new(async move {
                                    match (channels.await, categories.await)  {
                                        (Ok(channels), Ok(categories)) => {
                                            let channels_with_category: Store<HashMap<Uuid, Store<ChannelStore>>> = Store::new(HashMap::new());
                                            let general_channels = Store::new(ChannelStore { channels: vec![] });
                                            for category in &categories {
                                                channels_with_category.update(|channels| {
                                                    channels
                                                    .insert(category.id, Store::new(ChannelStore { channels: vec![] }));
                                                });
                                            }
                                            let categories = Store::new(CategoryStore { categories});
                                            for channel in channels {
                                                match channel.category_id {
                                                    Some(category_id) => {
                                                        channels_with_category.update(|channels| {
                                                            channels
                                                            .entry(category_id)
                                                            .and_modify(|store| store.channels().update(|channels| channels.push(channel)));
                                                        });
                                                    }
                                                    None => {
                                                        general_channels.update(|store| store.channels.push(channel));
                                                    }
                                                }
                                            }
                                            let sync =use_sync();
                                            if let Some(sync) = sync {
                                                sync.message_router.on_module_msg("ChannelStore", move |msg: ChannelStoreSync| {
                                                    match msg {
                                                        ChannelStoreSync::Deleted { id } => {
                                                            general_channels.channels().update(|channels| {
                                                                channels.retain(|channel| channel.id != id);
                                                            });
                                                            channels_with_category.update(|channels| {
                                                                 channels.iter().for_each(|(_, store)| {
                                                                    store.update(|store| {
                                                                        store.channels.retain(|channel| channel.id != id);
                                                                    });
                                                                });
                                                            });
                                                        },
                                                        ChannelStoreSync::Created { channel } => {
                                                            if let Some(category_id)= channel.category_id {
                                                                if let Some(channels) = channels_with_category.get().get(&category_id) { channels.update(|store| store.channels.push(channel)); }
                                                            } else {
                                                                general_channels.update(|store| store.channels.push(channel));
                                                            };
                                                        },
                                                        ChannelStoreSync::Updated { id } => {
                                                            debug!("Update for channel: {id}");
                                                        },
                                                    }
                                                });
                                                sync.message_router.on_module_msg("CategoryStore", move |msg: CategoryStoreSync| {
                                                    match msg {
                                                        CategoryStoreSync::Deleted { id } => {
                                                                            categories.update(|store| {
                                                                                store.categories.retain(|category| category.id != id);
                                                                            });

                                                                    channels_with_category.update(|store| {
                                                                        if let Some(store) = store.remove(&id) {
                                                                            store.channels().update(|channels| {
                                                                                channels.iter_mut().for_each(|channel| channel.category_id = None);
                                                                            });
                                                                            general_channels.channels().update(|channels| {
                                                                                channels.extend(store.channels().get())
                                                                            });
                                                                        }
                                                                    });
                                                        },
                                                        CategoryStoreSync::Created { category } => {
                                                            channels_with_category.update(|store| {
                                                                store.insert(category.id, Store::new(ChannelStore { channels: vec![] }));
                                                            });
                                                            categories.update(|store| store.categories.push(category));
                                                        },
                                                        CategoryStoreSync::Updated { id } => {
                                                            debug!("Update for category: {id}");
                                                        },
                                                    }

                                                });
                                            };

                                            view!{
                                                <For
                                                    each=move || general_channels.channels()
                                                    key=|channel| channel.id().get()
                                                    let:channel
                                                >
                                                    <Channel channel=channel />
                                                </For>
                                                <For
                                                    each=move || {
                                                        categories.categories()
                                                    }
                                                    key=|category| category.id().get()
                                                    children=move |category| {
                                                        let channels = *channels_with_category.get().get(&category.id().get()).expect("should exist the channels");
                                                        view!{
                                                            <Category category=category channels=channels/>
                                                        }
                                                    }
                                                />
                                            }.into_any()
                                        },
                                        _ => ().into_any()
                                    }
                                })}
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
