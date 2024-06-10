use crate::app::components::invite_people::InvitePeopleModal;
use crate::app::components::ui::dropdown_menu::*;
use crate::entities::member::Member;
use crate::entities::member::Role;
use crate::entities::server::Server;
use icondata;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn ServerMenu(server: Server, member: Member) -> impl IntoView {
    let open = create_rw_signal(false);
    view! {
        <DropdownProvider open=open>
            <DropdownTrigger class="relative w-full cursor-pointer">
                <div class="relative font-medium py-3 px-4 shadow shadow-base-300/80">
                    <div class="h-6 flex items-center">
                        <div class="mr-2"/>
                        <div class="flex-1 flex items-center text-base font-bold overflow-hidden text-ellipsis whitespace-nowrap min-w-0">
                            {server.name}
                        </div>
                        <Icon icon=icondata::RiArrowDownSArrowsLine class="relative"/>
                    </div>
                </div>
            </DropdownTrigger>/* transition ease-out */
            <DropdownContent class="transition-all ease-out w-[220px] flex flex-col h-auto py-[6px] px-2 bg-base-300 dark:bg-[#0d0d0d] z-40 rounded".to_string() side=MenuSide::Bottom side_of_set=12.0>
                <InvitePeopleModal invite_code=server.invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                    <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
                </InvitePeopleModal>
                <div class="divider my-0 mx-1 w-full"/>
                {
                    if let Role::ADMIN = member.role {
                        view! {
                            <ServerMenuAdminItems/>
                        }
                    } else {
                        ().into_view()
                    }
                }
                <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                    <div class="group-hover:text-primary-content">"Edit Server Profile"</div>
                    <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
                </div>
                {
                    if let Role::GUEST = member.role {
                        view! {
                            <ServerMenuGuestItems/>
                        }
                    } else {
                        ().into_view()
                    }
                }
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn ServerMenuAdminItems() -> impl IntoView {
    view! {
        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Server Settings"</div>
            <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </div>

        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Create Channel"</div>
            <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </div>

        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Create Category"</div>
            <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </div>
        <div class="divider my-0 mx-1 w-full"/>
    }
}

#[component]
pub fn ServerMenuGuestItems() -> impl IntoView {
    view! {
        <div class="divider my-0 mx-1 w-full"/>
        <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
            <div class="group-hover:text-primary-content">"Leave Server"</div>
            <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
        </div>
    }
}
