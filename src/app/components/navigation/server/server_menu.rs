use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::modal::leave_server::LeaveServer;
use crate::app::components::overview::server::ServerOverviewTrigger;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use icondata;
use leptos::html;
use leptos::prelude::*;
use leptos_icons::*;

#[component]
pub fn ServerMenu() -> impl IntoView {
    let open = RwSignal::new(false);
    let hidden = RwSignal::new(false);
    let on_click_item = Signal::derive(move || hidden.set(true));
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();
    let invite_people_node = NodeRef::<html::Div>::new();
    let create_channel_node = NodeRef::<html::Div>::new();
    let create_category_node = NodeRef::<html::Div>::new();
    let leave_server_node = NodeRef::<html::Div>::new();
    view! {
        <div class="relative w-full px-2 py-1.5">
            <DropdownProvider open=open modal=false hidden=hidden>
                <DropdownTrigger class="relative w-full cursor-pointer font-medium py-1 px-1 hover:bg-base-100 rounded-md">
                    <div class="h-6 flex items-center">
                        <div class="mr-1" />
                        <p class="block mr-auto text-base overflow-hidden font-semibold text-ellipsis whitespace-nowrap min-w-0">
                            {server.name}
                        </p>
                        <Icon icon=icondata::LuChevronDown />
                    // class="relative"
                    </div>
                </DropdownTrigger>
                <DropdownContent
                    ignore=vec![
                        invite_people_node,
                        create_channel_node,
                        create_category_node,
                        leave_server_node,
                    ]
                    class="w-auto h-auto z-40"
                    side=MenuSide::Bottom
                    side_of_set=2.0
                >
                    <div class="w-56 origin-top starting:opacity-0 starting:-translate-y-2 starting:scale-95 transition-all flex flex-col h-auto p-1 bg-base-300 rounded-md border border-base-100 select-none">
                        <InvitePeopleModal
                            content_ref=invite_people_node
                            invite_code=server.invite_code
                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                            on_click=Signal::derive(move || open.set(false))
                        >
                            <div>"Invite People"</div>
                        </InvitePeopleModal>
                        {if member_can_edit {
                            view! {
                                <div class="bg-base-100 h-px my-1 -mx-1" />
                                <ServerMenuAdminItems
                                    create_channel_node=create_channel_node
                                    create_category_node=create_category_node
                                    on_click=on_click_item
                                />
                            }
                                .into_any()
                        } else {
                            ().into_any()
                        }}
                        <div class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm">
                            <div>"Edit Server Profile"</div>
                        </div>
                        {if !member_can_edit {
                            view! {
                                <ServerMenuGuestItems
                                    leave_server_node=leave_server_node
                                    on_click=on_click_item
                                />
                            }
                                .into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                </DropdownContent>
            </DropdownProvider>
        </div>
    }
}

#[component]
fn ServerMenuAdminItems(
    on_click: Signal<()>,
    create_channel_node: NodeRef<html::Div>,
    create_category_node: NodeRef<html::Div>,
) -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();
    view! {
        <ServerOverviewTrigger
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
        >
            "Server Settings"
        </ServerOverviewTrigger>

        <CreateChannelModal
            content_ref=create_channel_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
        >
            <div>"Create Channel"</div>
        </CreateChannelModal>

        <CreateCategoryModal
            content_ref=create_category_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
        >
            <div>"Create Category"</div>
        </CreateCategoryModal>
        <div class="bg-base-100 h-px my-1 -mx-1" />
    }
}

#[component]
fn ServerMenuGuestItems(
    on_click: Signal<()>,
    leave_server_node: NodeRef<html::Div>,
) -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();
    view! {
        <div class="bg-base-100 h-px my-1 -mx-1" />
        <LeaveServer
            content_ref=leave_server_node
            server=server
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
            on_click=on_click
        >
            <div>"Leave Server"</div>
        </LeaveServer>
    }
}
