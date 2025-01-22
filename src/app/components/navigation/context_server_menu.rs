use crate::app::api::member::member_can_edit;
use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::modal::leave_server::LeaveServer;
use crate::app::components::ui::context_menu::*;
use crate::entities::server::Server;
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ContextServerMenu(
    server: Server,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let hidden = create_rw_signal(false);
    let on_click_item = Signal::derive(move || hidden.set(true));
    let member_can_update = create_resource(|| (), move |_| member_can_edit(server.id));
    let invite_code = server.invite_code;
    let server = store_value(server);
    let create_channel_node = create_node_ref::<html::Div>();
    let create_category_node = create_node_ref::<html::Div>();
    let invite_people_node = create_node_ref::<html::Div>();
    let leave_server_node = create_node_ref();

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
                class="transition-all ease-out w-56 flex flex-col h-auto p-1 bg-base-400 z-40 rounded-md border border-base-100"
                    .to_string()
            >
                <InvitePeopleModal
                    content_ref=invite_people_node
                    invite_code=invite_code
                    class="flex justify-between hover:bg-base-content/5 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                    on_click=on_click_item
                >
                    <div class="">"Invite People"</div>
                </InvitePeopleModal>
                <Transition fallback=move || ()>
                    {move || {
                        if let Some(Ok(true)) = member_can_update.get() {
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
                            }
                                .into_view()
                        } else {
                            ().into_view()
                        }
                    }}
                    {move || {
                        if let Some(Ok(false)) = member_can_update.get() {
                            view! {
                                <div class="bg-base-100 h-px my-1 -mx-1" />
                                <ServerMenuGuestItems
                                    leave_server_node=leave_server_node
                                    server=server.get_value()
                                    on_click=on_click_item
                                />
                            }
                                .into_view()
                        } else {
                            ().into_view()
                        }
                    }}
                </Transition>
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}

#[derive(Default, Copy, Clone)]
pub struct ServerMenuNodes {
    pub create_channel_node: NodeRef<html::Div>,
    pub create_category_node: NodeRef<html::Div>,
}

#[allow(non_snake_case)]
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
        <div class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm">
            <div class="">"Server Settings"</div>
        </div>

        <CreateChannelModal
            content_ref=create_channel_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
        >
            <div class="">"Create Channel"</div>
        </CreateChannelModal>

        <CreateCategoryModal
            content_ref=create_category_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
        >
            <div class="">"Create Category"</div>
        </CreateCategoryModal>
    }
}

#[allow(non_snake_case)]
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
            class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
            on_click=on_click
        >
            <div class="group-hover:text-primary-content">"Leave Server"</div>
        </LeaveServer>
    }
}
