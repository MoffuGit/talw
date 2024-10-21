use crate::app::api::member::{get_members_from_role, get_members_without_role};
use crate::app::api::server::get_server_roles;
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::SideBarContext;
use crate::app::components::ui::collapsible::*;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::Member;
use crate::entities::role::Role;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn ServerMemberSideBar(server_id: uuid::Uuid) -> impl IntoView {
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
                            let member_count = members.len();
                            let open = create_rw_signal(false);
                            view!{
                                <div class="h-full shrink-0 w-[240px] bg-base-300 flex flex-col items-stretch justify-between">
                                    <div class="flex flex-col overflow-y-scroll overflow-x-hidden items-stretch">
                                        {roles.iter().map(|role| view!{<Role role=role.clone()/>}).collect_view()}
                                        <CollapsibleProvider open=open>
                                            <CollapsibleTrigger class="pt-6 pr-2 pl-3 text-base cursor-pointer box-border flex items-center justify-between group">
                                                <div class="flex flex-auto overflow-hidden items-center">
                                                    <Icon icon=icondata::RiArrowDownSArrowsLine class=MaybeProp::derive(move || Some(TextProp::from(format!("h-4 w-4 text-base-content/75 group-hover:text-base-content {}", {
                                                        match open.get() {
                                                            true => "",
                                                            false =>"-rotate-90"
                                                        }
                                                    }))))/>
                                                    <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                                                        {format!("Online - {}", member_count)}
                                                    </div>
                                                </div>
                                            </CollapsibleTrigger>
                                            <CollapsibleContent>
                                                {members.iter().map(|member| view!{<Member member=member.clone()/>}).collect_view()}
                                            </CollapsibleContent>
                                        </CollapsibleProvider>
                                    </div>
                                    <CurrentMember member=use_current_server_context().member />
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
pub fn CurrentMember(member: Member) -> impl IntoView {
    let image_url = member.image_url.clone();
    let name = member.name.clone();
    view! {
        <MemberBanner side=MenuSide::Left align=MenuAlign::Start class="p-2 flex items-center bg-gradient-to-l from-base-100" member=member>
            {
                if let Some(url) = image_url {
                    view! {
                        <img class="rounded-full object-cover bg-base-100/80 w-9 h-9 mr-2" src=url/>
                    }.into_view()
                } else {
                    view! {
                        <div class="rounded-full bg-base-100/80 w-9 h-9 mr-2"/>
                    }.into_view()
                }
            }
            <div>
                {name}
            </div>
        </MemberBanner>
    }
}

#[component]
pub fn Role(role: Role) -> impl IntoView {
    let members = create_resource(|| (), move |_| get_members_from_role(role.id));
    let open = create_rw_signal(false);

    view! {
        {
            move || match members.get() {
                Some(Ok(members)) if !members.is_empty() => {
                    let name = role.name.clone();
                    let member_count = members.len();
                    view!{
                        <CollapsibleProvider open=open>
                            <CollapsibleTrigger class="pt-6 pr-2 pl-3 text-base cursor-pointer box-border flex items-center justify-between group">
                                <div class="flex flex-auto overflow-hidden items-center">
                                    <Icon icon=icondata::RiArrowDownSArrowsLine class=MaybeProp::derive(move || Some(TextProp::from(format!("h-4 w-4 text-base-content/75 group-hover:text-base-content {}", {
                                        match open.get() {
                                            true => "",
                                            false =>"-rotate-90"
                                        }
                                    }))))/>
                                    <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                                        {format!("{} - {}",&name, member_count)}
                                    </div>
                                </div>
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                {members.iter().map(|member| view!{<Member member=member.clone()/>}).collect_view()}
                            </CollapsibleContent>
                        </CollapsibleProvider>
                    }.into_view()

                },
                _ => ().into_view()
            }
        }
    }
}

#[component]
pub fn Member(member: Member) -> impl IntoView {
    // let member = store_value(member);
    let image_url = member.image_url.clone();
    let name = member.name.clone();
    view! {
        <MemberBanner side=MenuSide::Left align=MenuAlign::Start class="hover:bg-base-100/60 rounded mb-0.5 ml-3 mr-2 p-2 flex items-center" member=member>
            {
                if let Some(url) = image_url {
                    view! {
                        <img class="rounded-full object-cover bg-base-100/80 w-9 h-9 mr-2" src=url/>
                    }.into_view()
                } else {
                    view! {
                        <div class="rounded-full bg-base-100/80 w-9 h-9 mr-2"/>
                    }.into_view()
                }
            }
            <div>
                {name}
            </div>
        </MemberBanner>
    }
}
