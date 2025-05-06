use leptos::either::Either;
use leptos::prelude::*;
use reactive_stores::Field;

use crate::app::api::user::get_user_profile;
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::member::{Member as MemberEnt, MemberStoreFields};

#[component]
pub fn Member(#[prop(into)] member: Field<MemberEnt>) -> impl IntoView {
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
                                member=member
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
