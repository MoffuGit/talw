use leptos::*;

// use crate::app::components::modal::change_password::ChangePasswordModal;
// use crate::app::components::overview::user::UserOverviewContext;

#[component]
pub fn AuthenticationSettings() -> impl IntoView {
    //NOTE: check this again
    // let user = use_context::<UserOverviewContext>()
    //     .expect("should acces to the user overview context")
    //     .user;
    view! {
        <div class="font-bold text-xl mb-2">"Password and Authentication"</div>
        <div>"email"</div>
        <div>"number"</div>
        // <ChangePasswordModal user_id=user.id class="flex items-center bg-accent hover:bg-accent-focus text-accent-content items-center w-fit text-sm py-[6px] px-4 my-0.5 group rounded">
        //     <div>"Change Password"</div>
        // </ChangePasswordModal>
        <div>"Delete Account"</div>
    }
}
