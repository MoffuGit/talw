use crate::app::api::{auth::current_user, server::use_server};
use crate::app::components::create_server::use_create_server;
use crate::app::components::ui::modal::slide_modal::SlideBack;
use crate::app::components::ui::modal::ModalClose;
use crate::app::components::ui::modal::ModalProviderContext;
use icondata;
use leptos::{logging::warn, *};
use leptos_icons::*;
use leptos_router::ActionForm;

#[component]
pub fn Select_Name() -> impl IntoView {
    let create_server = use_server().create_server;
    let select_name_ref = use_create_server().select_name_ref;
    let user = move || {
        current_user().get().map(|user| match user {
            Ok(Some(user)) => format!("{}'s server", user.username),
            _ => "User".to_string(),
        })
    };

    view! {
        <Transition fallback=move || ()>
            <ActionForm action=create_server node_ref=select_name_ref>
                <div class="px-6 pt-6 relative flex flex-col justify-start items-center ">
                    <div class="text-center font-bold text-2xl leading-[30px]">Customize your server</div>
                    <div class="mt-2 text-center text-base leading-5 font-normal">Give your new server a personality with a name and an icon. You can always change it later.</div>
                    <ModalClose attr:type="reset" class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <div class="px-2 my-4">
                    <div class="pt-1 flex justify-center">
                        <div class="w-20 h-20"/>
                    </div>
                    <div class="mt-6">
                        {
                            move || {
                                create_server.value().get().map(|res| match res {
                                    Err(ServerFnError::ServerError(err)) => view! { <p class="text-error w-full text-center">{err}</p>},
                                    _ => view! { <p class="text-error w-full text-center"/>},
                                })
                            }
                        }
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
