use leptos::either::Either;
use leptos::prelude::*;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::MemberStoreFields;

#[component]
pub fn Profile() -> impl IntoView {
    let member = use_current_server_context().member;
    let image_url = member.image_url();
    let name = member.name();
    view! {
        {
            move || {
                member.with(|member| view!{
                    <MemberBanner
                        side=MenuSide::Left
                        align=MenuAlign::Start
                        class="p-2 relative flex items-center border-0 border-t border-t-base-100"
                        member=member.clone()
                    >
                        <div class="px-1 py-2 hover:bg-base-100 rounded-md flex items-center relative cursor-pointer select-none grow">
                            {move || if let Some(url) = image_url.get() {
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
                            }} <div>{move || name.get()}</div>
                        </div>
                    </MemberBanner>

                })
            }
        }
    }
}
