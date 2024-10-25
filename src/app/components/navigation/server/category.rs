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

    view! {
        <CollapsibleProvider open=collapsible_open>
            <CollapsibleTrigger class="relative mt-4 mb-0.5">
                <ContextMenuProvider modal=false hidden=hidden_context_menu>
                    <ContextMenuTrigger class="cursor-pointer box-border pr-2 pl-2 flex items-center justify-between group">
                        <div class="flex flex-auto overflow-hidden items-center">
                            <Icon icon=icondata::RiArrowDownSArrowsLine class=MaybeProp::derive(move || Some(TextProp::from(format!("h-4 w-4 text-base-content/75 group-hover:text-base-content {}", {
                                match collapsible_open.get() {
                                    true => "",
                                    false =>"-rotate-90"
                                }
                            }))))/>
                            <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                                {name.get_value()}
                            </div>
                            {
                                match member_can_edit {
                                    true => view!{
                                        <TooltipProvider delay_duration=Duration::new(0,5)>
                                            <TooltipTrigger class="w-auto h-auto">
                                                <CreateChannelModal server_id=server.id category_id=id class="w-auto h-auto" category_name=name.get_value()>
                                                    <Icon icon=icondata::RiAddSystemFill class="w-4 h-4 fill-base-content/75 group-hover:text-base-content"/>
                                                </CreateChannelModal>
                                            </TooltipTrigger>
                                            <TooltipContent tip="Add Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                        </TooltipProvider>
                                    }.into_view(),
                                    _ =>  view! {}.into_view()
                                }
                            }
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuContent ignore=vec![create_channel_node, edit_category_node, delete_category_node] class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                        <CreateChannelModal content_ref=create_channel_node server_id=server.id category_id=id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" category_name=name.get_value() on_click=Signal::derive(move || hidden_context_menu.set(false))>
                            <div class="group-hover:text-primary-content">"Create Channel"</div>
                        </CreateChannelModal>
                        <EditCategoryModal content_ref=edit_category_node category=category.get_value() on_click=Signal::derive(move || hidden_context_menu.set(false)) class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" >
                            <div class="group-hover:text-primary-content">"Rename Category"</div>
                        </EditCategoryModal>
                        <DeleteCategoryModal content_ref=delete_category_node category=category.get_value() server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || hidden_context_menu.set(false))>
                            <div class="group-hover:text-primary-content">"Delete Category"</div>
                        </DeleteCategoryModal>
                    </ContextMenuContent>
                </ContextMenuProvider>
            </CollapsibleTrigger>
            <CollapsibleContent>
            <Transition fallback=move || ()>
                {
                    move || {
                        channels.and_then(|channels| {
                            channels.iter().map(|channel| {
                                view! {<Channel channel=channel.clone() />}
                            }).collect_view()
                        })
                    }
                }
            </Transition>
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
