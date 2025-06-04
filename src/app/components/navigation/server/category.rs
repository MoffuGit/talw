use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::delete_category::DeleteCategoryModal;
use crate::app::components::modal::edit_category::EditCategoryModal;
use crate::app::components::navigation::server::channel::Channel;
use crate::app::components::navigation::server::sidebar::ChannelStore;
use crate::app::components::navigation::server::sidebar::ChannelStoreStoreFields;
use crate::app::components::ui::collapsible::*;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::icons::Icon;
use crate::app::components::ui::icons::IconData;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use crate::entities::category::CategoryStoreFields;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
// use crate::ws::client::use_ws;
use leptos::html;
use leptos::prelude::*;
use reactive_stores::Field;
use reactive_stores::Store;

use crate::entities::category::Category as EntCategory;

#[component]
pub fn Category(
    #[prop(into)] category: Field<EntCategory>,
    channels: Store<ChannelStore>,
) -> impl IntoView {
    let collapsible_open = RwSignal::new(false);
    let hidden_context_menu = RwSignal::new(false);

    let CurrentServerContext { server, .. } = use_current_server_context();

    let id = category.id();
    let name = category.name();

    let create_channel_node = NodeRef::<html::Div>::new();
    let edit_category_node = NodeRef::<html::Div>::new();
    let delete_category_node = NodeRef::<html::Div>::new();
    let menu_open = RwSignal::new(false);
    // let ws = use_ws();
    //
    // ws.on_server_msg(server.id().get(), move |msg| {
    //     if let Message::CategoryUpdated {
    //         category_id,
    //         new_name,
    //     } = msg
    //     {
    //         if category.id().get() == category_id {
    //             *category.name().write() = new_name;
    //         }
    //     }
    // });

    view! {
        <CollapsibleProvider open=collapsible_open>
            <ContextMenuProvider modal=false hidden=hidden_context_menu open=menu_open>
                <ContextMenuTrigger class="relative group mt-0.5 px-2 w-full select-none min-w-[240px]">
                    <div class="relative box-border flex flex-col cursor-pointer">
                        <CollapsibleTrigger class="relative flex group items-center py-1.5 px-2 rounded-md hover:bg-base-100 h-8">
                            <Icon icon=IconData::ChevronRight
                                class=Signal::derive(
                                    move || {
                                        format!(
                                            "h-4 w-4 text-base-content/75 group-hover:text-base-content mr-1.5 transition-transform {}",
                                            {
                                                match collapsible_open.get() {
                                                    true => "rotate-90",
                                                    false => "",
                                                }
                                            },
                                        )
                                    }
                                )
                            />
                            <div class="box-border ml-0.5 text-ellipsis text-sm whitespace-nowrap overflow-hidden leading-4 tracking-wide mr-auto">
                                {move || name.get()}
                            </div>
                        </CollapsibleTrigger>
                        <div
                            on:click=move |_| {
                                menu_open.set(true);
                            }
                            class="absolute right-1 top-1.5 p-0.5 hover:bg-base-100 rounded opacity-0 group-hover:opacity-100"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-ellipsis"
                            >
                                <circle cx="12" cy="12" r="1" />
                                <circle cx="19" cy="12" r="1" />
                                <circle cx="5" cy="12" r="1" />
                            </svg>
                        </div>
                    </div>
                </ContextMenuTrigger>
                <ContextMenuContent
                    ignore=vec![create_channel_node, edit_category_node, delete_category_node]
                    class="select-none z-40"
                >
                    <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                        <CreateChannelModal
                            content_ref=create_channel_node
                            server_id=server.id()
                            category_id=id
                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                            category_name=name
                            on_click=Signal::derive(move || hidden_context_menu.set(false))
                        >
                            <div>"Create Channel"</div>
                        </CreateChannelModal>
                        <EditCategoryModal
                            content_ref=edit_category_node
                            category=category
                            on_click=Signal::derive(move || hidden_context_menu.set(false))
                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                        >
                            <div>"Rename Category"</div>
                        </EditCategoryModal>
                        <DeleteCategoryModal
                            content_ref=delete_category_node
                            category=category
                            server_id=server.id().get()
                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                            on_click=Signal::derive(move || hidden_context_menu.set(false))
                        >
                            <div>"Delete Category"</div>
                        </DeleteCategoryModal>
                    </div>
                </ContextMenuContent>
            </ContextMenuProvider>
            <CollapsibleContent>
                <For
                    each=move || channels.channels()
                    key=|channel| channel.id().get()
                    let:channel
                >
                    <Channel channel=channel />
                </For>
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
