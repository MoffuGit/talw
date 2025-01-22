use crate::app::api::category::use_category;
use crate::app::api::channel::use_channel;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::delete_category::DeleteCategoryModal;
use crate::app::components::modal::edit_category::EditCategoryModal;
use crate::app::components::navigation::server::channel::Channel;
use crate::app::components::ui::collapsible::*;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use icondata;
use leptos::*;
use leptos_icons::*;
use std::time::Duration;

use crate::app::api::channel::get_channels_with_category;
use crate::entities::category::Category;

#[allow(non_snake_case)]
#[component]
pub fn Category(category: StoredValue<Category>) -> impl IntoView {
    let collapsible_open = create_rw_signal(false);
    let hidden_context_menu = create_rw_signal(false);
    let delete_category = use_category().delete_category;
    let create_channel_with_category = use_channel().create_channel_with_category;
    let delete_channel = use_channel().delete_channel;
    let rename_channel = use_channel().rename_channel;
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();

    let Category { id, name, .. } = category.get_value();
    let name = store_value(name);

    let channels = create_resource(
        move || {
            (
                create_channel_with_category.version().get(),
                delete_channel.version().get(),
                rename_channel.version().get(),
                delete_category.version().get(),
            )
        },
        move |_| get_channels_with_category(server.id, id),
    );

    let create_channel_node = create_node_ref::<html::Div>();
    let edit_category_node = create_node_ref::<html::Div>();
    let delete_category_node = create_node_ref::<html::Div>();
    let menu_open = create_rw_signal(false);

    view! {
        <CollapsibleProvider open=collapsible_open>
            <ContextMenuProvider modal=false hidden=hidden_context_menu open=menu_open>
                <ContextMenuTrigger class="relative mt-0.5 ml-2 py-px group">
                    <CollapsibleTrigger class="cursor-pointer box-border flex items-center justify-between">
                        <div class="flex flex-auto overflow-hidden items-center py-1.5 px-2 hover:bg-base-content/5 rounded-lg h-8">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=move || format!(
                                "h-4 w-4 text-base-content/75 group-hover:text-base-content mr-1.5 {}",
                                {
                                    match collapsible_open.get() {
                                        true => "rotate-90",
                                        false => "",
                                    }
                                },
                            )>
                                <path d="m9 18 6-6-6-6"/>
                            </svg>
                            <div class="box-border ml-0.5 text-ellipsis text-sm whitespace-nowrap overflow-hidden leading-4 tracking-wide mr-auto">
                                {name.get_value()}
                            </div>
                        </div>
                    </CollapsibleTrigger>
                    <div
                        on:click=move |_| {
                            menu_open.set(true);
                        }
                        class="absolute right-1 top-1.5 p-0.5 hover:bg-base-content/5 rounded opacity-0 group-hover:opacity-100"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-ellipsis"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>
                    </div>
                </ContextMenuTrigger>
                <ContextMenuContent
                    ignore=vec![create_channel_node, edit_category_node, delete_category_node]
                    class="transition-all ease-out w-56 flex flex-col h-auto p-1 bg-base-400 z-40 rounded-md border border-base-100"
                        .to_string()
                >
                    <CreateChannelModal
                        content_ref=create_channel_node
                        server_id=server.id
                        category_id=id
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        category_name=name.get_value()
                        on_click=Signal::derive(move || hidden_context_menu.set(false))
                    >
                        <div>"Create Channel"</div>
                    </CreateChannelModal>
                    <EditCategoryModal
                        content_ref=edit_category_node
                        category=category.get_value()
                        on_click=Signal::derive(move || hidden_context_menu.set(false))
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                    >
                        <div >"Rename Category"</div>
                    </EditCategoryModal>
                    <DeleteCategoryModal
                        content_ref=delete_category_node
                        category=category.get_value()
                        server_id=server.id
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on_click=Signal::derive(move || hidden_context_menu.set(false))
                    >
                        <div>"Delete Category"</div>
                    </DeleteCategoryModal>
                </ContextMenuContent>
            </ContextMenuProvider>
            <CollapsibleContent>
                <Transition fallback=move || ()>
                    {move || {
                        channels
                            .and_then(|channels| {
                                channels
                                    .iter()
                                    .map(|channel| {
                                        view! { <Channel channel=channel.clone() /> }
                                    })
                                    .collect_view()
                            })
                    }}
                </Transition>
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
