use super::slide_modal::SlideBack;
use super::ModalClose;
use crate::app::components::modals::ModalProviderContext;
use crate::app::{auth::current_user, server::use_server};
use leptos::{logging::warn, *};
use leptos_icons::RiIcon::*;
use leptos_icons::*;
use leptos_router::ActionForm;

#[component]
pub fn Select_Name() -> impl IntoView {
    let create_server = use_server().create_server;
    let user = move || {
        current_user().get().map(|user| match user {
            Ok(Some(user)) => format!("{}'s server", user.username),
            _ => "User".to_string(),
        })
    };
    let form_ref = create_node_ref::<html::Form>();
    let is_open = use_context::<ModalProviderContext>().expect("hae condafskl");

    create_effect(move |_| {
        if !is_open.get() {
            if let Some(form) = form_ref.get() {
                form.reset();
            }
        }
    });

    create_effect(move |_| {
        create_server
            .version()
            .with(|_| is_open.update(|value| *value = false))
    });

    view! {
        <Transition fallback=move || ()>
            <ActionForm action=create_server node_ref=form_ref>
                <div class="px-6 pt-6 relative flex flex-col justify-start items-center ">
                    <div class="text-center font-bold text-2xl leading-[30px]">Customize your server</div>
                    <div class="mt-2 text-center text-base leading-5 font-normal">Give your new server a personality with a name and an icon. You can always change it later.</div>
                    <ModalClose attr:type="reset" class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=Icon::from(RiCloseSystemLine) class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <div class="px-2 my-4">
                    <div class="pt-1 flex justify-center">
                        <div class="w-20 h-20"/>
                    </div>
                    <div class="mt-6">
                        <label class="mb-2 text-xs leading-4 font-bold">SERVER NAME</label>
                        <div class="flex flex-col">
                            <input name="name" type="text" class="input input-secondary font-medium p-[10px] h-10 text-base w-full rounded-[3px]" type="text" placeholder=user />
                        </div>
                    </div>
                    <div class="mt-2 pb-1 text-xs leading-4 font-normal">"By creating a server, you agree to Discord's Community Guidelines."</div>
                </div>
                <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-200">
                    <SlideBack attr:type="reset" class="w-auto min-h-min h-[38px] py-0.5 px-1 leading-[16px] hover:underline text-base-content text-[14px]">
                            Back
                    </SlideBack>
                    <button type="submit" class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation">Create Server</button>
                </div>
            </ActionForm>
        </Transition>
    }
}
