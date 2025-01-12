mod authentication;
mod profile;

use authentication::AuthenticationSettings;
use leptos::*;
use profile::ProfilesSettings;

#[component]
pub fn AccountSettings() -> impl IntoView {
    view! {
        <div class="relative w-full h-full flex flex-col items-start">
            <ProfilesSettings />
            <AuthenticationSettings />
        </div>
    }
}
