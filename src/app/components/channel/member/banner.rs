use crate::app::components::ui::dropdown_menu::*;
use crate::entities::member::Member;
use leptos::*;
#[component]
pub fn MemberBanner(
    side: MenuSide,
    align: MenuAlign,
    class: &'static str,
    children: Children,
    member: Member,
) -> impl IntoView {
    view! {
        <DropdownProvider modal=false>
            <DropdownTrigger class=class>
                {children()}
            </DropdownTrigger>
            <DropdownContent class="w-72 h-auto z-50 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded-lg".into() side=side align=align side_of_set=20.0>
                <div class="relative w-full h-auto">
                    <div class="w-full h-28 bg-red-500 rounded-t-lg"/>
                    <div class="w-[96px] h-[96px] absolute top-16 left-4 rounded-full bg-white border-[6px] border-[#dfdfe2] dark:border-[#0d0d0d]"/>
                    <div class="relative w-full mt-10">
                        <div>{&member.name}</div>
                    </div>
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}
