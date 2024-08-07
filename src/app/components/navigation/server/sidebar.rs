use super::category::Category;
use super::channel::Channel;
use super::server_menu::ServerMenu;
use crate::app::api::category::get_categories;
use crate::app::api::category::use_category;
use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::ui::context_menu::*;
use crate::{
    app::api::channel::{get_general_channels, use_channel},
    entities::{member::Member, server::Server},
};
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ServerSideBar(server: Server, member: Member) -> impl IntoView {
    let use_channel = use_channel();
    let create_channel = use_channel.create_channel;
    let delete_channel = use_channel.delete_channel;
    let rename_channel = use_channel.rename_channel;

    let use_category = use_category();
    let delete_category = use_category.delete_category;
    let create_category = use_category.create_category;
    let rename_category = use_category.rename_category;

    let channels = create_resource(
        move || {
            (
                delete_category.version().get(),
                create_channel.version().get(),
                delete_channel.version().get(),
                rename_channel.version().get(),
            )
        },
        move |_| get_general_channels(server.id),
    );

    let categories = create_resource(
        move || {
            (
                create_category.version().get(),
                delete_category.version().get(),
                rename_category.version().get(),
            )
        },
        move |_| get_categories(server.id),
    );
    let open = create_rw_signal(false);
    view! {
        <div class="w-full h-full flex flex-col items-center relative bg-base-200 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                <ServerMenu server=server.clone() member=member.clone()/>
                <div class="overflow-x-hidden overflow-y-scroll pr-2 flex-auto">
                    <div class="h-3"/>
                    <Transition fallback=move || ()>
                        {
                            move || {
                                channels.with(|channels| {
                                    match channels {
                                        Some(Ok(channels)) => {
                                            channels.iter().map(|channel| {
                                                view! {<Channel channel=channel.clone() invite_code=server.invite_code server_id=server.id member_role=member.role/>}
                                            }).collect_view()
                                        },
                                        _ => view!{<div/>}.into_view()
                                    }
                                })
                            }
                        }
                    </Transition>
                    <Transition fallback=move || ()>
                        {
                            move || {
                                categories.with(|categories| {
                                    match categories  {
                                        Some(Ok(categories)) => {
                                            categories.iter().map(|category| {
                                                view! {<Category category=category.clone() server_id=server.id invite_code=server.invite_code member_role=member.role/>}
                                            }).collect_view()
                                        },
                                        _ => view!{<div/>}.into_view()
                                    }
                                })
                            }
                        }
                    </Transition>
                </div>
            </div>
            <ContextMenuProvider modal=false open=open >
                <ContextMenuTrigger class="h-full w-full bg-none"/>
                <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                    <CreateChannelModal server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                        <div class="group-hover:text-primary-content">"Create Channel"</div>
                    </CreateChannelModal>
                    <CreateCategoryModal server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                        <div class="group-hover:text-primary-content">"Create Category"</div>
                    </CreateCategoryModal>
                </ContextMenuContent>
            </ContextMenuProvider>
        </div>
    }
}
