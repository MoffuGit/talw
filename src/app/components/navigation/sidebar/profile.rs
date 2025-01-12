use leptos::*;

use crate::app::api::user::use_user;
use crate::app::components::overview::user::UserOverviewTrigger;

#[component]
pub fn Profile() -> impl IntoView {
    let profile = use_user().profile;
    view! {
        <UserOverviewTrigger class="">
            <Transition>
                {move || {
                    profile
                        .and_then(|profile| {
                            view! {
                                {if let Some(url) = &profile.image_url {
                                    view! {
                                        <img
                                            class="w-[48px] h-[48px] rounded-full object-cover mx-2.5"
                                            src=url
                                        />
                                    }
                                        .into_view()
                                } else {
                                    view! {
                                        <div class="w-[48px] h-[48px] rounded-full bg-base-100/40 mx-2.5"></div>
                                    }
                                        .into_view()
                                }}
                            }
                        })
                }}
            </Transition>
        </UserOverviewTrigger>
    }
}
