use crate::app::api::server::get_server_roles;
use crate::app::api::thread::{
    get_thread_members_with_role, get_thread_members_without_role, use_thread,
};
use crate::app::api::user::get_user_profile;
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::server::CurrentMember;
use crate::app::components::channel::sidebars::SideBarContext;
use crate::app::components::ui::collapsible::*;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::member::Member;
use crate::entities::role::Role;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_icons::Icon;
use uuid::Uuid;

#[component]
pub fn ThreadMemberSideBar(server_id: Uuid, thread_id: Uuid) -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    let use_thread = use_thread();
    let roles = Resource::new(
        move || {
            (
                use_thread.join_thread.version().get(),
                use_thread.leave_thread.version().get(),
            )
        },
        move |_| get_server_roles(server_id),
    );
    let members_without_role = Resource::new(
        move || {
            (
                use_thread.join_thread.version().get(),
                use_thread.leave_thread.version().get(),
            )
        },
        move |_| get_thread_members_without_role(thread_id),
    );
    view! {
        <div
            class="h-full shrink-0 bg-base-300 flex flex-col items-stretch justify-between ease-linear duration-200 transition-[width]"
            style=move || if open.get() { "width: 240px" } else { "width: 0px" }
        >
            <Transition>
                <div class="flex flex-col overflow-y-scroll overflow-x-hidden items-stretch">
                    <For
                        each=move || roles.get().and_then(Result::ok).unwrap_or_default()
                        key=|role: &Role| role.id
                        children=move |role: Role| {
                            view! { <Role role=role.clone() thread_id=thread_id /> }
                        }
                    />
                    <CollapsibleProvider>
                        <div class="pt-6 px-2 select-none text-base cursor-pointer box-border flex items-center justify-between">
                            <CollapsibleTrigger class="flex flex-auto overflow-hidden items-center p-2 rounded-md hover:bg-base-100">
                                <Icon icon=icondata::LuChevronRight />
                                // class=MaybeProp::derive(move || Some(
                                // TextProp::from(
                                // format!(
                                // "h-4 w-4 {}",
                                // {
                                // match open.get() {
                                // true => "rotate-90",
                                // false => "",
                                // }
                                // },
                                // ),
                                // ),
                                // ))
                                {move || {
                                    members_without_role
                                        .and_then(|members| {
                                            view! {
                                                <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-semibold tracking-wide mr-auto">
                                                    {format!("Online - {}", members.len())}
                                                </div>
                                            }
                                        })
                                }}
                            </CollapsibleTrigger>
                        </div>
                        <CollapsibleContent>
                            <For
                                each=move || {
                                    members_without_role
                                        .get()
                                        .and_then(Result::ok)
                                        .unwrap_or_default()
                                }
                                key=|member: &Member| member.id
                                children=move |member: Member| {
                                    view! { <Member user_id=member.user_id member_id=member.id /> }
                                }
                            />
                        </CollapsibleContent>
                    </CollapsibleProvider>
                </div>
            </Transition>
            <CurrentMember />
        </div>
    }
}

#[component]
pub fn Role(role: Role, thread_id: Uuid) -> impl IntoView {
    let members = Resource::new(
        || (),
        move |_| get_thread_members_with_role(role.id, thread_id),
    );
    let open = RwSignal::new(false);

    view! {
        <Transition>
            <CollapsibleProvider open=open>
                <div class="pt-6 px-2 select-none text-base cursor-pointer box-border flex items-center justify-between">
                    <CollapsibleTrigger class="flex flex-auto overflow-hidden items-center p-2 rounded-md hover:bg-base-100">
                        <Icon icon=icondata::RiArrowDownSArrowsLine />
                        // class=MaybeProp::derive(move || Some(
                        // TextProp::from(
                        // format!(
                        // "h-4 w-4 text-base-content/75 group-hover:text-base-content {}",
                        // {
                        // match open.get() {
                        // true => "",
                        // false => "-rotate-90",
                        // }
                        // },
                        // ),
                        // ),
                        // ))
                        <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content mr-auto">
                            {move || {
                                members
                                    .and_then(|members| {
                                        view! {
                                            <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-semibold tracking-wide mr-auto">
                                                {format!("{} - {}", role.name, members.len())}
                                            </div>
                                        }
                                    })
                            }}
                        </div>
                    </CollapsibleTrigger>
                </div>
                <CollapsibleContent>
                    <For
                        each=move || members.get().and_then(Result::ok).unwrap_or_default()
                        key=|member: &Member| member.id
                        children=move |member: Member| {
                            view! { <Member member_id=member.id user_id=member.user_id /> }
                        }
                    />
                </CollapsibleContent>
            </CollapsibleProvider>
        </Transition>
    }
}

#[component]
pub fn Member(user_id: Uuid, member_id: Uuid) -> impl IntoView {
    let profile = Resource::new(move || (), move |_| get_user_profile(user_id));
    view! {
        <Transition>
            {move || {
                profile
                    .and_then(|profile| {
                        let name = StoredValue::new(profile.name.clone());
                        let image_url = profile.image_url.clone();
                        view! {
                            <MemberBanner
                                side=MenuSide::Left
                                align=MenuAlign::Start
                                class="hover:bg-base-100 rounded-md mb-0.5 ml-3 mr-2 p-2 flex items-center select-none cursor-pointer"
                                member_id=member_id
                                user_id=user_id
                                profile=profile.clone()
                            >
                                {if let Some(url) = image_url {
                                    Either::Left(
                                        view! {
                                            <img
                                                class="rounded-full object-cover w-9 h-9 mr-2"
                                                src=url
                                            />
                                        },
                                    )
                                } else {
                                    Either::Right(
                                        view! {
                                            <div class="rounded-full bg-base-content/10 w-9 h-9 mr-2" />
                                        },
                                    )
                                }}
                                <div>{move || name.get_value()}</div>
                            </MemberBanner>
                        }
                    })
            }}
        </Transition>
    }
}
