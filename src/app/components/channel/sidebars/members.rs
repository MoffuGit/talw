use crate::app::api::server::get_members_without_role;
use crate::app::api::server::{get_members_from_role, get_server_roles};
use crate::app::components::channel::member::banner::*;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::tool_tip::{
    ToolTipSide, TooltipContent, TooltipProvider, TooltipTrigger,
};
use crate::entities::member::Member;
use crate::entities::role::Role;
use icondata;
use leptos::*;
use leptos_icons::Icon;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SideBarContext(pub RwSignal<bool>);

#[allow(non_snake_case)]
#[component]
pub fn MemberSideBar(server_id: uuid::Uuid) -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    let roles = create_resource(|| (), move |_| get_server_roles(server_id));
    let members_without_role = create_resource(|| (), move |_| get_members_without_role(server_id));
    view! {
        <Transition fallback=move || ()>
            {
                move || {
                    match (open.get(), roles.get(), members_without_role.get()) {
                        (true, Some(Ok(roles)), Some(Ok(members))) => {
                            view!{
                                <div class="h-full shrink-0 w-[240px] bg-base-300 flex flex-col overflow-y-scroll overflow-x-hidden items-stretch justify-start">
                                    {roles.iter().map(|role| view!{<Role role=role.clone()/>}).collect_view()}
                                    <div class="pt-6 pr-2 pl-4 text-base">{format!("Online - {}", members.len())}</div>
                                    {members.iter().map(|member| view!{<Member member=member.clone()/>}).collect_view()}
                                </div>
                            }.into_view()
                        }
                        _ => ().into_view()
                    }
                }
            }
        </Transition>
    }
}

#[component]
pub fn Role(role: Role) -> impl IntoView {
    let members = create_resource(|| (), move |_| get_members_from_role(role.id));

    view! {
        {
            move || match members.get() {
                Some(Ok(members)) if !members.is_empty() => {
                    view!{
                        <div class="pt-6 pr-2 pl-4 text-base">{format!("{} - {}", role.name, members.len())}</div>
                        {members.iter().map(|member| view!{<Member member=member.clone()/>}).collect_view()}
                    }.into_view()

                },
                _ => ().into_view()
            }
        }
    }
}

#[component]
pub fn Member(member: Member) -> impl IntoView {
    let member = store_value(member);
    view! {
        <MemberBanner side=MenuSide::Left align=MenuAlign::Start class="hover:bg-base-100/60 rounded mb-0.5 ml-4 mr-2 p-2 flex items-center" member=member.get_value()>
            <div class="rounded-full bg-base-100 w-8 h-8 mr-2"/>
            <div>
                { member.get_value().name}
            </div>
        </MemberBanner>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MemberSideBarTrigger() -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger close_on_click=false on_click=Signal::derive(move || open.update(|open| *open = !*open))>
                <Icon icon=icondata::RiGroup2UserFacesFill class="w-6 h-6" />
            </TooltipTrigger>
            <TooltipContent tooltip_of_side=10.0 tip=Signal::derive(move || match open.get() { true => "Hide Members SideBar".to_string() , false => "Show Members SideBar".to_string()} )  tooltip_side=ToolTipSide::Bottom class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-b-[#dfdfe2] dark:after:border-b-[#0d0d0d]" />
        </TooltipProvider>
    }
}
