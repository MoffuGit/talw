use leptos::*;

use crate::entities::user::User;

#[component]
pub fn Profile(user: User) -> impl IntoView {
    view! {
        {
            if let Some(url) = user.image_url {
                view!{
                    <img class="w-[48px] h-[48px] rounded-full object-cover mx-2.5" src=url/>
                }.into_view()
            } else {
                view!{
                    <div class="w-[48px] h-[48px] rounded-full bg-base-100/40 mx-2.5">
                    </div>
                }.into_view()
            }
        }
    }
}
