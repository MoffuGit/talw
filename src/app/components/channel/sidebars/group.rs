use leptos::either::Either;
use leptos::prelude::*;
use reactive_stores::Store;
use uuid::Uuid;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::channel::sidebars::collapsible::Collapsible;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::routes::servers::{MemberStore, MemberStoreStoreFields};
use crate::entities::member::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Groups {
    Online(Option<Uuid>),
    Offline,
}

#[component]
pub fn Group(
    members: Store<MemberStore>,
    #[prop(into)] name: Signal<String>,
    group: Groups,
) -> impl IntoView {
    let group = Memo::new(move |_| {
        members
            .members()
            .get()
            .into_iter()
            .filter(|member| match group {
                Groups::Online(role_id) => {
                    member.status == Status::ONLINE && member.role_id == role_id
                }
                Groups::Offline => member.status == Status::OFFLINE,
            })
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
