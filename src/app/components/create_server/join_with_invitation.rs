use crate::app::api::server::{use_server, JoinServerWithInvitation};
use crate::app::components::ui::modal::slide_modal::SlideBack;
use crate::app::components::ui::modal::ModalClose;
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::ActionForm;

#[component]
pub fn Join_with_invitation() -> impl IntoView {
    let use_server = use_server();
    view! {
        <Transition fallback=move || ()>
            <ActionForm action=use_server.join_with_invitation>
                <div class="text-center p-[16px]">
                    <h1 class="font-bold mb-2 mt-6 text-[24px] leading-[30px]">Join a Server</h1>
                    <div class="text-[14px] leading-[18px]">Enter an invite below to join an existing server</div>
                    <ModalClose attr:type="reset" class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <div class="px-4">
                    <div class="mb-4">
                        <div>
                                {move || {
                                    use_server.join_with_invitation.value().get().map(|res| {
                    log::info!("{res:?}");
                    match res {
                                        Err(ServerFnError::ServerError(err)) => view! { <p class="text-error w-full text-center">{err}</p>},
                                        _ => view! { <p class="text-error w-full text-center"/>},
                                    }
                })
                                        }
                                }
                            <h2 class="mb-2 text-[12px] leading-[16px] font-bold">INVITE LINK</h2>
                            <input type="text" name="invitation" class="input rounded input-secondary bg-secondary h-[40px] text-[14px] w-full" placeholder="https://discord.gg/hTKzmak"/>
                        </div>
                    </div>
                    <div class="mb-4">
                        <h2 class="mb-2 text-[12px] leading-[16px] font-bold">INVITE SHOULD LOOK LIKE</h2>
                        <div class="text-[14px] leading-[18px]">hTKzmak</div>
                        <div class="text-[14px] leading-[18px]">"https://discord.gg/hTKzmak"</div>
                        <div class="text-[14px] leading-[18px]">"https://discord.gg/cool-people"</div>
                    </div>
                    <div class="rounded-lg bg-base-200 mb-4 flex items-center p-3 text-left">
                        <div class="mr-3 w-[40px] h-[40px]"/>
                        <div>
                            <h2 class="text-[16px] leading-[20px] font-bold">"Don't have an invite?"</h2>
                            <div class="text-[12px] leading-[16px] font-normal">Check out Discoverable communities in Server Discovery.</div>
                        </div>
                        <div class="ml-auto mr-3">">"</div>
                    </div>
                </div>
                <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-200">
                    <SlideBack attr:type="reset" class="w-auto min-h-min h-[38px] py-0.5 px-1 leading-[16px] hover:underline text-base-content text-[14px]">
                            Back
                    </SlideBack>
                    <button type="submit" class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation">Join Server</button>
                </div>
            </ActionForm>
        </Transition>
    }
}
