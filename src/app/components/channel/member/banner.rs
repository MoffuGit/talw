use crate::app::api::member::get_member_roles;
use crate::app::api::user::{get_mutual_servers_image_url, get_user_banner};
use crate::app::components::overview::user::UserOverviewTrigger;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::{Member, MemberStoreFields};
use crate::entities::user::BannerStoreFields;
use leptos::either::Either;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_document;
use reactive_stores::Store;
use uuid::Uuid;

#[component]
pub fn MemberBanner(
    side: MenuSide,
    align: MenuAlign,
    class: &'static str,
    children: Children,
    member: Member,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let content_ref: NodeRef<Div> = NodeRef::new();
    let limit_y = Signal::derive(move || {
        let content_height = {
            if let Some(node) = content_ref.get() {
                node.offset_height() as f64
            } else {
                320.0
            }
        };
        use_document()
            .body()
            .map(|body| (body.get_bounding_client_rect().height() - content_height) - 20.0)
            .unwrap_or(320.0)
    });
    let name = StoredValue::new(member.name);
    let image_url = StoredValue::new(member.image_url);
    let user_member = use_current_server_context().member.id();
    view! {
        <DropdownProvider content_ref=content_ref modal=false open=is_open>
            <DropdownTrigger class=class>{children()}</DropdownTrigger>
            <DropdownContent
                class="w-72 h-auto z-50"
                side=side
                align=align
                side_of_set=20.0
                limit_y=limit_y
            >
                <Show when=move || {
                    is_open.get()
                }>
                    {move || {
                        let banner = Resource::new(move || (member.user_id), get_user_banner);
                        view! {
                            <Transition>
                                {
                                    Suspend::new(async move {
                                        banner.await.map(|banner| {
                                            let banner = Store::new(banner.clone());
                                            let about = banner.about();
                                            let banner_url = banner.image_url();
                                            view! {
                                                <div class=format!("relative w-full h-full flex flex-col select-none w-56 {} starting:opacity-0 starting:translate-x-2 starting:scale-95 transition-all rounded-md bg-base-300", match side {
                                                    MenuSide::Bottom =>  "origin-top",
                                                    MenuSide::Left => "origin-right",
                                                    MenuSide::Right => "origin-left",
                                                    MenuSide::Top => "origin-bottom",
                                                })>
                                                    {move || {
                                                        if let Some(url) = banner_url.get() {
                                                            Either::Left(
                                                                view! {
                                                                    <img
                                                                        class="w-full h-28 object-cover rounded-t-lg"
                                                                        src=url
                                                                    />
                                                                },
                                                            )
                                                        } else {
                                                            Either::Right(
                                                                view! {
                                                                    <div class="w-full h-28 bg-base-primary rounded-t-lg" />
                                                                },
                                                            )
                                                        }
                                                    }}
                                                    {if let Some(url) = image_url.get_value() {
                                                        Either::Left(
                                                            view! {
                                                                <img
                                                                    class="w-[96px] h-[96px] object-cover absolute top-16 left-2 rounded-full border-[6px] border-base-300"
                                                                    src=url
                                                                />
                                                            },
                                                        )
                                                    } else {
                                                        Either::Right(
                                                            view! {
                                                                <div class="w-[96px] h-[96px] absolute top-16 left-2 rounded-full border-[6px] bg-base-content/10 border-base-300" />
                                                            },
                                                        )
                                                    }} <div class="relative w-auto mt-14 m-4">
                                                        <div class="text-xl font-semibold">{name.get_value()}</div>
                                                        <MutualServers user_id=member.user_id/>
                                                        <MemberRoles member_id=member.id/>
                                                        {move || {
                                                            about
                                                                .get()
                                                                .map(|about| {
                                                                    view! { <div>{about}</div> }
                                                                })
                                                        }}
                                                        {move || {
                                                            if member.id != user_member.get() {
                                                                Either::Left(
                                                                    view! {
                                                                        <div class="flex mt-4 border border-base-100 hover:bg-base-content/10 rounded-md w-full h-12 px-4 items-center cursor-pointer">
                                                                            <div class="text-base">
                                                                                {format!("Message @{}", name.get_value())}
                                                                            </div>
                                                                        </div>
                                                                    },
                                                                )
                                                            } else {
                                                                Either::Right(
                                                                    view! {
                                                                        <UserOverviewTrigger class="flex mt-4 rounded-md border border-base-100 hover:bg-base-content/10 w-full h-12 px-4 items-center cursor-pointer">
                                                                            "Open Settings"
                                                                        </UserOverviewTrigger>
                                                                    },
                                                                )
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            }

                                        })
                                    })
                                }
                            </Transition>
                        }
                    }}
                </Show>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn MutualServers(user_id: Uuid) -> impl IntoView {
    let mutual_servers = Resource::new(
        || (),
        move |_| async move {
            get_mutual_servers_image_url(user_id)
                .await
                .map(|servers| servers.into_iter().enumerate().collect::<Vec<_>>())
        },
    );
    view! {
        <Transition>
            <div class="flex items-center mt-4">
                <div class="flex justify-start -space-x-2 mr-1">
                    <For
                        each=move || mutual_servers.get().and_then(Result::ok).unwrap_or_default()
                        key=|(idx, _)| *idx
                        children=move |(_, url)| {
                            if let Some(url) = url {
                                Either::Left(
                                    view! {
                                        <img class="w-4 h-4 object-cover rounded-full" src=url />
                                    },
                                )
                            } else {
                                Either::Right(
                                    view! {
                                        <div class="w-4 h-4 rounded-full bg-white border-base-200" />
                                    },
                                )
                            }
                        }
                    />
                </div>
                <div class="text-sm">
                    {move || {
                        mutual_servers
                            .and_then(|mutual| {
                                match mutual.len() {
                                    0 => None,
                                    1 => Some("1 Mutual Server".to_string()),
                                    many => Some(format!("{many} Mutuals Servers")),
                                }
                            })
                    }}
                </div>
            </div>
        </Transition>
    }
}

#[component]
pub fn MemberRoles(member_id: Uuid) -> impl IntoView {
    let member_roles = Resource::new(|| (), move |_| get_member_roles(member_id));
    view! {
        <Transition fallback=move || ()>
            {move || {
                member_roles
                    .and_then(|role| {
                        if !role.is_empty() {
                            view! { <div>{role.len()}</div> }.into_any()
                        } else {
                            ().into_any()
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn About(about: Option<String>) -> impl IntoView {
    about.map(|about| {
        view! { <div>{about}</div> }
    })
}
