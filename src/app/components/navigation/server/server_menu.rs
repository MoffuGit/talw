use crate::app::components::ui::dropdown_menu::*;
use crate::entities::member::Member;
use icondata;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn ServerMenu(server_name: String, member: Member) -> impl IntoView {
    view! {
        <DropdownProvider>
            <DropdownTrigger class="relative w-full cursor-pointer">
                <div class="relative font-medium py-3 px-4 shadow shadow-base-300/80">
                    <div class="h-6 flex items-center">
                        <div class="mr-2"/>
                        <div class="flex-1 flex items-center text-base font-bold overflow-hidden text-ellipsis whitespace-nowrap min-w-0">
                            {server_name}
                        </div>
                        <Icon icon=icondata::RiArrowDownSArrowsLine class="relative"/>
                    </div>
                </div>
            </DropdownTrigger>/* transition ease-out */
            <DropdownContent class=" w-[220px] flex h-auto py-[6px] px-2 bg-[#c6d2d2] dark:bg-[#0d0d0d] z-40 rounded".to_string() side=MenuSide::Bottom side_of_set=12.0>
                <div class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                    <Icon icon=icondata::RiTeamUserFacesFill class="w-[18px] h-[18px] ml-2 group-hover:fill-primary-content"/>
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}
