use crate::app::api::server::get_server_roles;
use crate::app::api::thread::{
    get_thread_members_with_role, get_thread_members_without_role, use_thread,
};
use crate::app::api::user::get_user_profile;
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::server::CurrentMember;
use crate::app::components::channel::sidebars::SideBarContext;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::role::Role;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn ThreadMemberSideBar(server_id: Uuid, thread_id: Uuid) -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    let use_thread = use_thread();
    let roles = create_resource(
        move || {
            (
                use_thread.join_thread.version().get(),
                use_thread.leave_thread.version().get(),
            )
        },
        move |_| get_server_roles(server_id),
    );
    let members_without_role = create_resource(
        move || {
            (
                use_thread.join_thread.version().get(),
                use_thread.leave_thread.version().get(),
            )
        },
        move |_| get_thread_members_without_role(thread_id),
    );
    view! {
        <Transition fallback=move || ()>
            {move || {
                match (open.get(), roles.get(), members_without_role.get()) {
                    (true, Some(Ok(roles)), Some(Ok(members))) => {
                        view! {
                            <div class="h-full shrink-0 w-[240px] bg-base-300 flex flex-col items-stretch justify-between">
                                <div class="flex flex-col items-stretch overflow-y-scroll overflow-x-hidden">
                                    {roles
                                        .iter()
                                        .map(|role| {
                                            view! { <Role role=role.clone() thread_id=thread_id /> }
                                        })
                                        .collect_view()}
                                    <div class="pt-6 pr-2 pl-4 text-base">
                                        {format!("Online - {}", members.len())}
                                    </div>
                                    {members
                                        .iter()
                                        .map(|member| {
                                            view! {
                                                <Member member_id=member.id user_id=member.user_id />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                                <CurrentMember />
                            </div>
                        }
                            .into_view()
                    }
                    _ => ().into_view(),
                }
            }}
        </Transition>
    }
}

#[component]
pub fn Role(role: Role, thread_id: Uuid) -> impl IntoView {
    let members = create_resource(
        || (),
        move |_| get_thread_members_with_role(role.id, thread_id),
    );

    view! {
        {move || match members.get() {
            Some(Ok(members)) if !members.is_empty() => {
                view! {
                    <div class="pt-6 pr-2 pl-3 text-base">
                        {format!("{} - {}", role.name, members.len())}
                    </div>
                    {members
                        .iter()
                        .map(|member| {
                            view! { <Member member_id=member.id user_id=member.user_id /> }
                        })
                        .collect_view()}
                }
                    .into_view()
            }
            _ => ().into_view(),
        }}
    }
}

#[component]
pub fn Member(user_id: Uuid, member_id: Uuid) -> impl IntoView {
    let profile = create_resource(move || (), move |_| get_user_profile(user_id));
    view! {
        <Transition fallback=move || ()>
            {move || {
                profile
                    .and_then(|profile| {
                        let image_url = profile.image_url.clone();
                        let name = profile.name.clone();
                        view! {
                            <MemberBanner
                                side=MenuSide::Left
                                align=MenuAlign::Start
                                class="hover:bg-base-100/60 rounded mb-0.5 ml-3 mr-2 p-2 flex items-center"
                                member_id=member_id
                                user_id=user_id
                                profile=profile.clone()
                            >
                                {if let Some(url) = image_url {
                                    view! {
                                        <img
                                            class="rounded-full object-cover bg-base-100/80 w-9 h-9 mr-2"
                                            src=url
                                        />
                                    }
                                        .into_view()
                                } else {
                                    view! {
                                        <div class="rounded-full bg-base-100/80 w-9 h-9 mr-2" />
                                    }
                                        .into_view()
                                }}
                                <div>{name}</div>
                            </MemberBanner>
                        }
                    })
            }}
        </Transition>
    }
}
