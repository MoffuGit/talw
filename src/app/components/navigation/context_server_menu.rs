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
            <ContextMenuTrigger class="flex overflow-hidden mx-3 h-[48px] transition-all items-center justify-center bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px]">
                {children.map(|children| children())}
            </ContextMenuTrigger>
            <ContextMenuContent
                ignore=vec![
                    leave_server_node,
                    create_channel_node,
                    create_category_node,
                    invite_people_node,
                ]
                class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] z-40 rounded"
                    .to_string()
            >
                <InvitePeopleModal
                    content_ref=invite_people_node
                    invite_code=invite_code
                    class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"
                    on_click=on_click_item
                >
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                </InvitePeopleModal>
                <Transition fallback=move || ()>
                    {move || {
                        if let Some(Ok(true)) = member_can_update.get() {
                            view! {
                                <div class="divider relative my-0 mx-1 w-auto" />
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
                                <div class="divider relative my-0 mx-1 w-auto" />
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
        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Server Settings"</div>
        </div>

        <CreateChannelModal
            content_ref=create_channel_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"
        >
            <div class="group-hover:text-primary-content">"Create Channel"</div>
        </CreateChannelModal>

        <CreateCategoryModal
            content_ref=create_category_node
            on_click=on_click
            server_id=server.id
            class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"
        >
            <div class="group-hover:text-primary-content">"Create Category"</div>
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
        <div class="divider relative my-0 mx-1 w-auto" />
        <LeaveServer
            content_ref=leave_server_node
            server=server
            class="flex justify-between text-error hover:text-error-content hover:bg-error items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"
            on_click=on_click
        >
            <div class="group-hover:text-primary-content">"Leave Server"</div>
        </LeaveServer>
    }
}
