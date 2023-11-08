use super::use_slides;
use crate::app::auth::current_user;
use leptos::*;
// use leptos_router::ActionForm;

#[component]
pub fn Select_Name() -> impl IntoView {
    let slides = use_slides();
    let user = move || {
        current_user().get().map(|user| match user {
            Ok(Some(user)) => format!("{}'s server", user.username),
            _ => "User".to_string(),
        })
    };

    view! {
        // <ActionForm>
            <div class="px-6 pt-6 relative flex flex-col justify-start items-center">
                <div class="text-center font-bold text-2xl leading-[30px]">Customize your server</div>
                <div class="mt-2 text-center text-base leading-5 font-normal">Give your new server a personality with a name and an icon. You can always change it later.</div>
                <form method="dialog">
                    <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">"✕"</button>
                </form>
            </div>
            <div class="px-2 my-4">
                <div class="pt-1 flex justify-center">
                    <div class="w-20 h-20"/>
                </div>
                <div class="mt-6">
                    <label class="mb-2 text-xs leading-4 font-bold">SERVER NAME</label>
                    <div class="flex flex-col">
                        <input class="input input-secondary font-medium p-[10px] h-10 text-base w-full rounded-[3px]" type="text" placeholder=user />
                    </div>
                </div>
                <div class="mt-2 pb-1 text-xs leading-4 font-normal">"By creating a server, you agree to Discord's Community Guidelines."</div>
            </div>
            <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-200">
                <button on:click=move |_| slides.update(|slides| {slides.pop();})  class="leading-[16px]  w-auto min-h-min hover:underline text-base-content h-[38px] text-[14px] py-0.5 px-1">Back</button>
                <button type="submit" class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation">Create Server</button>
            </div>
        // </ActionForm>
    }
}
