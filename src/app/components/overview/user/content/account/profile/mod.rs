mod banner_image;
mod user_about;
mod user_image;
mod user_name;

use banner_image::ImageBanner;
use leptos::prelude::*;

use user_image::UserImage;

use crate::app::api::user::use_user;
use crate::entities::user::{Banner, Profile};

use self::user_about::UserAbout;
use self::user_name::UserName;

#[derive(Clone)]
pub struct ProfilesSettingsContext {
    profile: Profile,
    banner: Banner,
    user_data_change: RwSignal<bool>,
}

#[component]
pub fn ProfilesSettings() -> impl IntoView {
    let user_context = use_user();
    let profile = user_context.profile;
    let banner = user_context.banner;
    view! {
        <div class="font-bold text-xl mb-2">"My Profile"</div>
        <Transition fallback=move || ()>
            {move || {
                match (profile.get(), banner.get()) {
                    (Some(Ok(profile)), Some(Ok(banner))) => {
                        view! { <UserBanner banner=banner profile=profile /> }.into_any()
                    }
                    _ => {
                        ().into_any()
                    },
                }
            }}
        </Transition>
    }
}

#[component]
fn UserBanner(banner: Banner, profile: Profile) -> impl IntoView {
    let primary_color_preview = RwSignal::new(banner.primary_color.clone());
    let user_data_change = RwSignal::new(false);
    //NOTE: Move the colors preview to its own component, this should wrapp the others and create
    //two components to select the color, one for primary, another for accent
    provide_context(ProfilesSettingsContext {
        user_data_change,
        profile,
        banner,
    });

    view! {
        <div class="relative w-[600px] bg-base-300 flex flex-col rounded-lg p-1.5 bg-gradient-to-b">
            <ImageBanner primary_color_preview=primary_color_preview />
            <UserImage />
            <ActionForm action=use_user().edit_user_data>
                // class="flex flex-col items-start w-full relative pb-16"
                <UserName />
                <UserAbout />
                <button
                    type="submit"
                    class=move || {
                        format!(
                            "absolute btn btn-primary rounded-md w-18 h-8 min-h-0 btn-sm bottom-4 right-8 {}",
                            { if user_data_change.get() { "visible" } else { "invisible" } },
                        )
                    }
                >
                    "Save"
                </button>
            // <input
            // type="color"
            // class="w-10 h-10"
            // on:input=move |evt| primary_color_preview.set(Some(event_target_value(&evt)))
            // />
            // <input
            // type="color"
            // class="w-10 h-10"
            // on:input=move |evt| accent_color_preview.set(Some(event_target_value(&evt)))
            // />
            </ActionForm>
        </div>
    }
}
