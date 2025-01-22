use crate::app::api::member::get_member_roles;
use crate::app::api::user::{get_mutual_servers_image_url, get_user_banner};
use crate::app::components::overview::user::UserOverviewTrigger;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::user::Profile;
use leptos::*;
use leptos_use::use_document;
use uuid::Uuid;

#[component]
pub fn MemberBanner(
    side: MenuSide,
    align: MenuAlign,
    class: &'static str,
    children: Children,
    member_id: Uuid,
    user_id: Uuid,
    profile: Profile,
) -> impl IntoView {
    let is_open = create_rw_signal(false);
    let limit_y = use_document()
        .body()
        .map(|body| body.get_bounding_client_rect().height() - 320.0);
    let name = store_value(profile.name);
    let image_url = store_value(profile.image_url);
    let user_member = use_current_server_context().member.id;
    view! {
        <DropdownProvider modal=false open=is_open>
            <DropdownTrigger class=class>{children()}</DropdownTrigger>
            <DropdownContent
                class="w-72 h-auto z-50 rounded-lg bg-base-300"
                side=side
                align=align
                side_of_set=20.0
                limit_y=limit_y
            >
                {move || {
                    if is_open.get() {
                        let banner = create_resource(move || (), move |_| get_user_banner(user_id));
                        view! {
                            <Transition>
                                {move || {
                                    banner
                                        .and_then(|banner| {
                                            let about = store_value(banner.about.clone());
                                            let banner_url = store_value(banner.image_url.clone());
                                            view! {
                                                <div class="relative w-full h-auto select-none">
                                                    {if let Some(url) = banner_url.get_value() {
                                                        view! {
                                                            <img
                                                                class="w-full h-28 object-cover rounded-t-lg"
                                                                src=url
                                                            />
                                                        }
                                                            .into_view()
                                                    } else {
                                                        view! {
                                                            <div class="w-full h-28 bg-base-primary rounded-t-lg" />
                                                        }
                                                            .into_view()
                                                    }}
                                                    {if let Some(url) = image_url.get_value() {
                                                        view! {
                                                            <img
                                                                class="w-[96px] h-[96px] object-cover absolute top-16 left-2 rounded-full border-[6px] border-base-300"
                                                                src=url
                                                            />
                                                        }
                                                            .into_view()
                                                    } else {
                                                        view! {
                                                            <div class="w-[96px] h-[96px] absolute top-16 left-2 rounded-full border-[6px] bg-base-content/10 border-base-300" />
                                                        }
                                                            .into_view()
                                                    }}
                                                    <div class="relative w-auto mt-14 m-4">
                                                        <div class="text-xl font-semibold">
                                                            {name.get_value()}
                                                        </div>
                                                        <MutualServers user_id=user_id />
                                                        <MemberRoles member_id=member_id />
                                                        {if let Some(about) = about.get_value() {
                                                            view! { <div>{about}</div> }.into_view()
                                                        } else {
                                                            ().into_view()
                                                        }}
                                                        {
                                                            move || if member_id != user_member {
                                                                view!{
                                                                    <div class="flex mt-4 border border-base-100 hover:bg-base-content/10 rounded-md w-full h-12 px-4 items-center cursor-pointer">
                                                                        <div class="text-base">
                                                                            {format!("Message @{}", name.get_value())}
                                                                        </div>
                                                                    </div>
                                                                }.into_view()
                                                            } else {
                                                                view!{
                                                                    <UserOverviewTrigger
                                                                        class="flex mt-4 rounded-md border border-base-100 hover:bg-base-content/10 w-full h-12 px-4 items-center cursor-pointer">
                                                                            "Open Settings"
                                                                    </UserOverviewTrigger>
                                                                }.into_view()
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            }
                                        })
                                }}
                            </Transition>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                }}
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn MutualServers(user_id: Uuid) -> impl IntoView {
    let mutual_servers = create_resource(|| (), move |_| get_mutual_servers_image_url(user_id));
    view! {
        <Transition fallback=move || ()>
            {move || {
                mutual_servers
                    .and_then(|mutual| {
                        let share = mutual.len();
                        if share > 0 {

                            view! {
                                <div class="flex items-center mt-4">
                                    <div class="flex justify-start -space-x-2 mr-1">
                                        {mutual
                                            .iter()
                                            .map(|url| {
                                                if let Some(url) = url {
                                                    view! {
                                                        <img class="w-4 h-4 object-cover rounded-full" src=url />
                                                    }
                                                        .into_view()
                                                } else {
                                                    view! {
                                                        <div class="w-4 h-4 rounded-full bg-white border-base-200" />
                                                    }
                                                        .into_view()
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                    <div class="text-sm">
                                        {format!(
                                            "{} {}",
                                            share,
                                            match share {
                                                1 => "Mutual Server",
                                                _ => "Mutuals Servers",
                                            },
                                        )}
                                    </div>
                                </div>
                            }
                                .into_view()
                        } else {
                            ().into_view()
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn MemberRoles(member_id: Uuid) -> impl IntoView {
    let member_roles = create_resource(|| (), move |_| get_member_roles(member_id));
    view! {
        <Transition fallback=move || ()>
            {move || {
                member_roles
                    .and_then(|role| {
                        if !role.is_empty() {
                            view! { <div>{role.len()}</div> }.into_view()
                        } else {
                            ().into_view()
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn About(about: Option<String>) -> impl IntoView {
    view! {
        {if let Some(about) = &about {
            view! { <div>{about}</div> }.into_view()
        } else {
            ().into_view()
        }}
    }
}
