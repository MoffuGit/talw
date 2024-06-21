use crate::app::api::server::use_server;
use crate::app::components::create_channel::CreateChannelModal;
use crate::app::components::delete_category::DeleteCategoryModal;
use crate::app::components::edit_category::EditCategoryModal;
use crate::app::components::navigation::server::channel::Channel;
use crate::app::components::ui::collapsible::*;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use crate::entities::member::Role;
use icondata;
use leptos::*;
use leptos_icons::*;
use std::time::Duration;
use uuid::Uuid;

use crate::{app::api::server::get_channels_with_category, entities::category::Category};

#[component]
pub fn Category(
    category: Category,
    server_id: Uuid,
    invite_code: Uuid,
    member_role: Role,
) -> impl IntoView {
    let collapsible_open = create_rw_signal(false);
    let context_menu_open = create_rw_signal(false);
    //NOTE: agregar subs to resource
    let delete_category = use_server().delete_category;
    let create_channel_with_category = use_server().create_channel_with_category;
    let delete_channel = use_server().delete_channel;
    let rename_channel = use_server().rename_channel;

    let channels = create_resource(
        move || {
            (
                create_channel_with_category.version().get(),
                delete_channel.version().get(),
                rename_channel.version().get(),
                delete_category.version().get(),
            )
        },
        move |_| get_channels_with_category(server_id, category.id),
    );
    let category = store_value(category);

    view! {
        <CollapsibleProvider open=collapsible_open>
            <CollapsibleTrigger class="relative mt-4 mb-0.5">
                <ContextMenuProvider modal=false open=context_menu_open>
                    <ContextMenuTrigger class="cursor-pointer box-border pr-2 pl-2 flex items-center justify-between group">
                        <div class="flex flex-auto overflow-hidden items-center">
                            <Icon icon=icondata::RiArrowDownSArrowsLine class=MaybeProp::derive(move || Some(TextProp::from(format!("h-4 w-4 text-base-content/75 group-hover:text-base-content {}", {
                                match collapsible_open.get() {
                                    true => "",
                                    false =>"-rotate-90"
                                }
                            }))))/>
                            <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                                {&category.get_value().name}
                            </div>
                            {
                                match member_role {
                                    Role::ADMIN => view!{
                                        <TooltipProvider delay_duration=Duration::new(0,5)>
                                            <TooltipTrigger class="w-auto h-auto">
                                                <CreateChannelModal server_id=server_id category_id=category.get_value().id class="w-auto h-auto" category_name=category.get_value().name.clone()>
                                                    <Icon icon=icondata::RiAddSystemFill class="w-4 h-4 fill-base-content/75 group-hover:text-base-content"/>
                                                </CreateChannelModal>
                                            </TooltipTrigger>
                                            <TooltipContent tip="Add Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                        </TooltipProvider>
                                    }.into_view(),
                                    Role::GUEST =>  view! {}.into_view()
                                }
                            }
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                        <CreateChannelModal server_id=server_id category_id=category.get_value().id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" category_name=category.get_value().name.clone() on_click=Signal::derive(move || context_menu_open.set(false))>
                            <div class="group-hover:text-primary-content">"Create Channel"</div>
                        </CreateChannelModal>
                        <EditCategoryModal category=category.get_value() on_click=Signal::derive(move || context_menu_open.set(false)) class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" >
                            <div class="group-hover:text-primary-content">"Rename Category"</div>
                        </EditCategoryModal>
                        <DeleteCategoryModal category=category.get_value() server_id=server_id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || context_menu_open.set(false))>
                            <div class="group-hover:text-primary-content">"Delete Category"</div>
                        </DeleteCategoryModal>
                    </ContextMenuContent>
                </ContextMenuProvider>
            </CollapsibleTrigger>
            <CollapsibleContent>
                {
                    move || {
                        channels.and_then(|channels| {
                            channels.iter().map(|channel| {
                                view! {<Channel channel=channel.clone() invite_code=invite_code server_id=server_id member_role=member_role/>}
                            }).collect_view()
                        })
                    }
                }
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
