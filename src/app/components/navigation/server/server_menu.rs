use crate::app::components::modal::create_category::CreateCategoryModal;
use crate::app::components::modal::create_channel::CreateChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::modal::leave_server::LeaveServer;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use icondata;
use leptos::*;
use leptos_icons::*;

#[allow(non_snake_case)]
#[component]
pub fn ServerMenu() -> impl IntoView {
    let open = create_rw_signal(false);
    let hidden = create_rw_signal(false);
    let on_click_item = Signal::derive(move || hidden.set(true));
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();
    let invite_people_node = create_node_ref::<html::Div>();
    let create_channel_node = create_node_ref::<html::Div>();
    let create_category_node = create_node_ref::<html::Div>();
    let leave_server_node = create_node_ref::<html::Div>();
    view! {
        <DropdownProvider open=open modal=false hidden=hidden>
            <DropdownTrigger class="relative w-full cursor-pointer">
                <div class="relative font-medium py-3 px-4 shadow shadow-base-300/80">
                    <div class="h-6 flex items-center">
                        <div class="mr-2"/>
                        <p class="block mr-auto text-base overflow-hidden font-bold text-ellipsis whitespace-nowrap min-w-0">
                            {server.name}
                        </p>
                        {
                            move || {
                                match open.get() {
                                    true => view! { <Icon icon=icondata::RiCloseSystemLine class="relative"/> },
                                    false => view! { <Icon icon=icondata::RiArrowDownSArrowsLine class="relative"/> }
                                }
                            }
                        }
                    </div>
                </div>
            </DropdownTrigger>/* transition ease-out */
            <DropdownContent ignore=vec![
                invite_people_node,
                create_channel_node,
                create_category_node,
                leave_server_node,
            ] class=" transition-transform scale-100 origin-top w-[220px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] z-40 rounded" side=MenuSide::Bottom side_of_set=12.0>
                <div class="transition-transform scale-100 origin-top">
                    <InvitePeopleModal content_ref=invite_people_node invite_code=server.invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                        <div class="group-hover:text-primary-content">"Invite People"</div>
                        <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
                    </InvitePeopleModal>
                    {
                        if member_can_edit {
                            view! {
                                <div class="divider relative my-0 mx-1 w-auto"/>
                                <ServerMenuAdminItems create_channel_node=create_channel_node create_category_node=create_category_node on_click=on_click_item/>
                            }.into_view()
                        } else {
                            ().into_view()
                        }
                    }
                    <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                        <div class="group-hover:text-primary-content">"Edit Server Profile"</div>
                        <Icon icon=icondata::RiPencilDesignFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
                    </div>
                    {
                        if !member_can_edit {
                            view! {
                                <ServerMenuGuestItems leave_server_node=leave_server_node on_click=on_click_item/>
                            }
                        } else {
                            ().into_view()
                        }
                    }
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[allow(non_snake_case)]
#[component]
fn ServerMenuAdminItems(
    on_click: Signal<()>,
    create_channel_node: NodeRef<html::Div>,
    create_category_node: NodeRef<html::Div>,
) -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();
    view! {
        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Server Settings"</div>
            <Icon icon=icondata::RiSettings5SystemFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </div>

        <CreateChannelModal content_ref=create_channel_node on_click=on_click server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Create Channel"</div>
            <Icon icon=icondata::RiAddCircleSystemFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </CreateChannelModal>

        <CreateCategoryModal content_ref=create_category_node on_click=on_click server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                <div class="group-hover:text-primary-content">"Create Category"</div>
                <Icon icon=icondata::RiFolderAddDocumentFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </CreateCategoryModal>
        <div class="divider relative my-0 mx-1 w-auto"/>
    }
}

#[allow(non_snake_case)]
#[component]
fn ServerMenuGuestItems(
    on_click: Signal<()>,
    leave_server_node: NodeRef<html::Div>,
) -> impl IntoView {
    let CurrentServerContext { server, .. } = use_current_server_context();
    view! {
        <div class="divider relative my-0 mx-1 w-auto"/>
        <LeaveServer content_ref=leave_server_node server=server class="flex justify-between text-error hover:text-error-content hover:bg-error items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=on_click>
            <div class="group-hover:text-primary-content">"Leave Server"</div>
            <Icon icon=icondata::RiDoorOpenOthersFill class="w-[18px] h-[18px] ml-2 fill-error group-hover:fill-error-content"/>
        </LeaveServer>
    }
}
