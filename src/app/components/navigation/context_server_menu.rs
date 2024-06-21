use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::modal::leave_server::LeaveServer;
use crate::app::components::ui::context_menu::*;
use crate::entities::member::Member;
use crate::entities::member::Role;
use crate::entities::server::Server;
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ContextServerMenu(
    server: Server,
    member: Member,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let server = store_value(server);
    let on_click_item = Signal::derive(move || open.set(false));
    view! {
        <ContextMenuProvider open=open modal=false>
            <ContextMenuTrigger class="flex mx-3 h-[48px] transition-all items-center justify-center bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px]">
                {children.map(|children| children())}
            </ContextMenuTrigger>
            <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] z-40 rounded".to_string()>
                <InvitePeopleModal invite_code=server.get_value().invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                </InvitePeopleModal>
                {
                    if let Role::ADMIN = member.role {
                        view! {
                            <div class="divider relative my-0 mx-1 w-auto"/>
                            <ServerMenuAdminItems server=server.get_value().clone() on_click=on_click_item/>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                }
                //NOTE: only ui not functioanl
                <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                    <div class="group-hover:text-primary-content">"Edit Server Profile"</div>
                </div>
                {
                    if let Role::GUEST = member.role {
                        view! {
                            <ServerMenuGuestItems server=server.get_value().clone() on_click=on_click_item/>
                        }
                    } else {
                        ().into_view()
                    }
                }
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}

#[allow(non_snake_case)]
#[component]
fn ServerMenuAdminItems(on_click: Signal<()>, server: Server) -> impl IntoView {
    view! {
        //NOTE: only ui not functioanl
        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Server Settings"</div>
        </div>

        <CreateChannelModal on_click=on_click server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Create Channel"</div>
        </CreateChannelModal>

        <CreateCategoryModal on_click=on_click server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                <div class="group-hover:text-primary-content">"Create Category"</div>
        </CreateCategoryModal>
        <div class="divider relative my-0 mx-1 w-auto"/>
    }
}

#[allow(non_snake_case)]
#[component]
fn ServerMenuGuestItems(server: Server, on_click: Signal<()>) -> impl IntoView {
    view! {
        <div class="divider relative my-0 mx-1 w-auto"/>
        <LeaveServer server=server class="flex justify-between text-error hover:text-error-content hover:bg-error items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=on_click>
            <div class="group-hover:text-primary-content">"Leave Server"</div>
        </LeaveServer>
    }
}
