use crate::app::api::member::member_can_edit;
use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::modal::leave_server::LeaveServer;
use crate::app::components::overview::server::ServerOverviewTrigger;
use crate::app::components::ui::context_menu::*;
use crate::entities::server::Server;
use leptos::either::Either;
use leptos::{html, prelude::*};

#[component]
pub fn ContextServerMenu(
    server: Server,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let hidden = RwSignal::new(false);
    let on_click_item = Signal::derive(move || hidden.set(true));
    let member_can_update = Resource::new(|| (), move |_| member_can_edit(server.id));
    let invite_code = server.invite_code;
    let server = StoredValue::new(server);
    let create_channel_node = NodeRef::<html::Div>::new();
    let create_category_node = NodeRef::<html::Div>::new();
    let invite_people_node = NodeRef::<html::Div>::new();
    let leave_server_node = NodeRef::new();

    view! {
        <ContextMenuProvider open=open modal=false hidden=hidden>
            <ContextMenuTrigger class="flex overflow-hidden w-8 h-8 transition-all items-center justify-center bg-base-100 text-base-content rounded-full group-hover:bg-primary group-hover:rounded-lg">
                {children.map(|children| children())}
            </ContextMenuTrigger>
            <ContextMenuContent
                ignore=vec![
                    leave_server_node,
                    create_channel_node,
                    create_category_node,
                    invite_people_node,
                ]
                class="select-none z-40"
            >
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300  rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <InvitePeopleModal
                        content_ref=invite_people_node
                        invite_code=invite_code
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                        on_click=on_click_item
                    >
                        <div class="">"Invite People"</div>
                    </InvitePeopleModal>
                    <Transition>
                        {move || {
                            member_can_update
                                .and_then(|can_edit| {
                                    if *can_edit {
                                        Either::Left(
                                            view! {
                                                <div class="bg-base-100 h-px my-1 -mx-1" />
                                                <ServerMenuAdminItems
                                                    nodes=ServerMenuNodes {
                                                        create_channel_node,
                                                        create_category_node,
                                                    }
                                                    server=server.get_value()
                                                    on_click=on_click_item
                                                />
                                            },
                                        )
                                    } else {
                                        Either::Right(
                                            view! {
                                                <div class="bg-base-100 h-px my-1 -mx-1" />
                                                <ServerMenuGuestItems
                                                    leave_server_node=leave_server_node
                                                    server=server.get_value()
                                                    on_click=on_click_item
                                                />
                                            },
                                        )
                                    }
                                })
                        }}
                    </Transition>
                </div>
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}

#[derive(Default, Copy, Clone)]
pub struct ServerMenuNodes {
    pub create_channel_node: NodeRef<html::Div>,
    pub create_category_node: NodeRef<html::Div>,
}

#[component]
fn ServerMenuAdminItems(
    on_click: Signal<()>,
    server: Server,
    nodes: ServerMenuNodes,
) -> impl IntoView {
    let ServerMenuNodes {
        create_channel_node,
        create_category_node,
    } = nodes;
    view! {
        <ServerOverviewTrigger
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
        >
            "Server Settings"
        </ServerOverviewTrigger>

        <CreateChannelModal
            content_ref=create_channel_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
        >
            <div class="">"Create Channel"</div>
        </CreateChannelModal>

        <CreateCategoryModal
            content_ref=create_category_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
        >
            <div class="">"Create Category"</div>
        </CreateCategoryModal>
    }
}

#[component]
fn ServerMenuGuestItems(
    server: Server,
    on_click: Signal<()>,
    leave_server_node: NodeRef<html::Div>,
) -> impl IntoView {
    view! {
        <div class="bg-base-100 h-px my-1 -mx-1" />
        <LeaveServer
            content_ref=leave_server_node
            server=server
            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
            on_click=on_click
        >
            <div class="group-hover:text-primary-content">"Leave Server"</div>
        </LeaveServer>
    }
}
