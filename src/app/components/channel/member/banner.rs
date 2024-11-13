use crate::app::api::member::get_member_roles;
use crate::app::api::user::{get_mutual_servers_image_url, get_user_about};
use crate::app::components::ui::dropdown_menu::*;
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
    name: String,
    image_url: Option<String>,
) -> impl IntoView {
    let is_open = create_rw_signal(false);
    let limit_y = use_document()
        .body()
        .map(|body| body.get_bounding_client_rect().height() - 320.0);
    let name = store_value(name);
    view! {
        <DropdownProvider modal=false open=is_open>
            <DropdownTrigger class=class>
                {children()}
            </DropdownTrigger>
            <DropdownContent class="w-72 h-auto z-50 rounded-lg bg-base-200" side=side align=align side_of_set=20.0 limit_y=limit_y>
                <div class="relative w-full h-auto ">
                    <div class="w-full h-28 bg-primary rounded-t-lg"/>
                        {
                            if let Some(url) = &image_url {
                                view! {
                                    <img class="w-[96px] h-[96px] object-cover absolute top-16 left-2 rounded-full border-[6px] border-base-200" src=url/>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="w-[96px] h-[96px] absolute top-16 left-2 rounded-full border-[6px] bg-base-100 border-base-200"/>
                                }.into_view()
                            }
                        }
                    <div class="relative w-auto mt-12 m-4">
                        <div class="text-xl font-bold bg-base-200">{name.get_value()}</div>
                        {
                            move || {
                                if is_open.get() {
                                    view!{
                                        <div class="text-base">{name.get_value()}</div>
                                        <MutualServers user_id=user_id/>
                                        <About user_id=user_id/>
                                        <MemberRoles member_id=member_id/>
                                    }.into_view()
                                } else {
                                    ().into_view()
                                }
                            }
                        }
                        <div class="flex mt-4 bg-base-300 rounded-lg w-full h-12 px-4 items-center">
                            <div class="text-base bg-base-300/80 text-base-content/60">
                                {
                                    format!("Message @{}", name.get_value())
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn MutualServers(user_id: Uuid) -> impl IntoView {
    let mutual_servers = create_resource(|| (), move |_| get_mutual_servers_image_url(user_id));
    view! {
        <Transition fallback=move || ()>
            {
                move || mutual_servers.and_then(|mutual| {
                    let share = mutual.len();

                    if share > 0 {
                        view!{
                            <div class="flex items-center mt-4">
                                <div class="flex justify-start -space-x-2 mr-1">
                                    {
                                        mutual.iter().map(|url| {
                                            if let Some(url) = url {
                                                view! {
                                                    <img class="w-4 h-4 object-cover rounded-full" src=url/>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <div class="w-4 h-4 rounded-full bg-white border-base-200"/>
                                                }.into_view()
                                            }
                                        }).collect_view()
                                    }
                                </div>
                                <div class="text-sm">{format!("{} {}", share, match share {
                                    1 => "Mutual Server",
                                    _ => "Mutuals Servers"
                                })}</div>
                            </div>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                })
            }
        </Transition>
    }
}

#[component]
pub fn MemberRoles(member_id: Uuid) -> impl IntoView {
    let member_roles = create_resource(|| (), move |_| get_member_roles(member_id));
    view! {
        <Transition fallback=move || ()>
            {
                move || member_roles.and_then(|role| {
                    if !role.is_empty() {
                        view!{<div>{role.len()}</div>}.into_view()
                    } else {
                        ().into_view()
                    }
                })
            }
        </Transition>
    }
}

#[component]
pub fn About(user_id: Uuid) -> impl IntoView {
    let about = create_resource(|| (), move |_| get_user_about(user_id));
    view! {
        <Transition fallback=move || ()>
            {
                move || about.and_then(|about| {
                    if let Some(about) = &about {
                        view!{
                            <div>{about}</div>
                        }.into_view()
                    } else {
                    ().into_view()
                }
                }).into_view()
            }
        </Transition>
    }
}
