use crate::app::api::server::get_server_roles;
use crate::app::api::thread::{
    get_thread_members_with_role, get_thread_members_without_role, use_thread,
};
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::SideBarContext;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::member::Member;
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
            {
                move || {
                    match (open.get(), roles.get(), members_without_role.get()) {
                        (true, Some(Ok(roles)), Some(Ok(members))) => {
                            view!{
                                <div class="h-full shrink-0 w-[240px] bg-base-300 flex flex-col overflow-y-scroll overflow-x-hidden items-stretch justify-start">
                                    {roles.iter().map(|role| view!{<Role role=role.clone() thread_id=thread_id/>}).collect_view()}
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
pub fn Role(role: Role, thread_id: Uuid) -> impl IntoView {
    let members = create_resource(
        || (),
        move |_| get_thread_members_with_role(role.id, thread_id),
    );

    view! {
        {
            move || match members.get() {
                Some(Ok(members)) if !members.is_empty() => {
                    view!{
                        <div class="pt-6 pr-2 pl-3 text-base">{format!("{} - {}", role.name, members.len())}</div>
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
    // let member = store_value(member);
    let image_url = member.image_url.clone();
    let name = member.name.clone();
    view! {
        <MemberBanner side=MenuSide::Left align=MenuAlign::Start class="hover:bg-base-100/60 rounded mb-0.5 ml-3 mr-2 p-2 flex items-center" member=member>
            {
                if let Some(url) = image_url {
                    view! {
                        <img class="rounded-full object-cover bg-base-100/80 w-8 h-8 mr-2" src=url/>
                    }.into_view()
                } else {
                    view! {
                        <div class="rounded-full bg-base-100/80 w-8 h-8 mr-2"/>
                    }.into_view()
                }
            }
            <div>
                {name}
            </div>
        </MemberBanner>
    }
}
