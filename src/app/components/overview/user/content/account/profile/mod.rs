mod banner_image;
mod user_about;
mod user_image;
mod user_name;

use banner_image::ImageBanner;
use leptos::*;
use user_image::UserImage;

use crate::app::api::user::use_user;
use crate::entities::user::{Banner, Profile};

use self::user_about::UserAbout;
use self::user_name::UserName;

#[derive(Clone)]
pub struct ProfilesSettingsContext {
    profile: Profile,
    banner: Banner,
}

#[component]
pub fn ProfilesSettings() -> impl IntoView {
    let user_context = use_user();
    let profile = user_context.profile;
    let banner = user_context.banner;
    view! {
        <div class="font-bold text-xl mb-2">"My Profile"</div>
        <Transition fallback=move || ()>
            {
                move || {
                    match (profile.get(), banner.get()) {
                        (Some(Ok(profile)), Some(Ok(banner))) => {
                            view!{
                                <UserBanner banner=banner profile=profile/>
                            }.into_view()
                    },
                        _ => view!{}.into_view()
                    }
                }
            }
        </Transition>
    }
}

#[component]
fn UserBanner(banner: Banner, profile: Profile) -> impl IntoView {
    let primary_color_preview = create_rw_signal(banner.primary_color.clone());
    provide_context(ProfilesSettingsContext { profile, banner });
    view! {
        <div class="relative w-full h-fit flex flex-col bg-base-200 rounded">
            <ImageBanner primary_color_preview=primary_color_preview/>
            <UserImage />
            <UserName/>
            <UserAbout/>
            <div class="">"Profile theme"</div>
            <div>"You cant change the profile theme for now"</div>
        </div>
    }
}
