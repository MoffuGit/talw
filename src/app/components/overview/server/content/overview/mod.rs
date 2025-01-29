mod server_image;
mod server_name;
use leptos::*;

use self::server_image::ServerImage;
use self::server_name::ServerName;

#[component]
pub fn OverviewSettings() -> impl IntoView {
    view! {
        <div class="relative w-full h-full flex flex-col items-start">
            <div class="font-bold text-xl mb-3">"Server Data"</div>
            <div class="relative w-full h-auto flex items-center">
                <ServerImage/>
                <ServerName/>
            </div>
        </div>
    }
}
