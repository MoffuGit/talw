use leptos::either::Either;
use leptos::prelude::*;

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
                            if let Some(url) = &profile.image_url {
                                Either::Left(
                                    view! {
                                        <img
                                        class="w-8 h-8 rounded-full cursor-pointer object-cover"
                                        src=url.clone()
                                        />
                                    }
                                )
                            } else {
                                Either::Right(
                                    view! {
                                        <div class="w-8 h-8 rounded-full cursor-pointer bg-base-100/40"/>
                                    }
                                )
                            }
                        })
                }}
            </Transition>
        </UserOverviewTrigger>
    }
}
