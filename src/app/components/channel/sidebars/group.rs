use std::collections::HashMap;

use leptos::either::Either;
use leptos::prelude::*;
use uuid::Uuid;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::collapsible::Collapsible;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::member::{Member, Status};

#[component]
pub fn Pagination(
    members: RwSignal<HashMap<Uuid, Member>>,
    pagination: Resource<Result<Vec<Member>, ServerFnError>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Transition>
            {
                Suspend::new(async move {
                    let _ = pagination.await.map(|page| {
                        members.update(|members| {
                            for member in page {
                                members.insert(member.user_id,member);
                            }
                        });
                    });
                    view!{
                        {children()}
                    }
                })
            }
        </Transition>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Groups {
    Online(Option<Uuid>),
    Offline,
}

#[component]
pub fn Group(
    members: RwSignal<HashMap<Uuid, Member>>,
    #[prop(into)] name: Signal<String>,
    group: Groups,
) -> impl IntoView {
    let group = Memo::new(move |_| {
        members
            .get()
            .values()
            .filter(|member| match group {
                Groups::Online(role_id) => {
                    member.status == Status::ONLINE && member.role_id == role_id
                }
                Groups::Offline => member.status == Status::OFFLINE,
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    view! {
        <Collapsible msg=Signal::derive(move || format!("{} - {}", name.get(), group.get().len()))>
            <For
                each=move || group.get()
                key=|member| member.id
                children=move |member| {
                    view!{
                        <MemberBanner
                            side=MenuSide::Left
                            align=MenuAlign::Start
                            class="hover:bg-base-100 rounded-md mb-0.5 ml-3 mr-2 p-2 flex items-center select-none cursor-pointer"
                            member=member.clone()
                        >
                            {if let Some(url) = member.image_url {
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
                            <div>{member.name}</div>
                        </MemberBanner>
                    }
                }
            />
        </Collapsible>
    }
}
