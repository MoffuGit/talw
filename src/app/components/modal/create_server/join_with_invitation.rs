use super::use_create_server;
use crate::app::api::server::use_server;
use crate::app::components::ui::modal::slide_modal::SlideBack;
use crate::app::components::ui::modal::ModalClose;
//use icondata;
use leptos::prelude::*;
//use leptos_icons::*;
use leptos_router::components::A;

#[component]
pub fn JoinWithInvitation() -> impl IntoView {
    let join_with_invitation = use_server().join_with_invitation;
    let use_create_server = use_create_server();
    let join_with_invitation_ref = use_create_server.join_with_invitation_ref;
    Effect::new(move |_| {
        join_with_invitation
            .version()
            .with(|_| use_create_server.is_open.set(false));
    });
    view! {
        <Transition fallback=move || ()>
            <ActionForm action=join_with_invitation node_ref=join_with_invitation_ref>
                <div class="text-center p-[16px]">
                    <h1 class="font-bold mb-2 mt-6 text-[24px] leading-[30px]">Join a Server</h1>
                    <div class="text-[14px] leading-[18px]">
                        Enter an invite below to join an existing server
                    </div>
                    <ModalClose
                        attr:r#type="reset"
                        class="absolute right-2 top-2 flex items-center group bg-none"
                    >
                        <div/>
                        // <Icon icon=icondata::RiCloseSystemLine />
                    // class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"
                    </ModalClose>
                </div>
                <div class="px-4">
                    <div class="mb-4">
                        <div>
                            {move || {
                                join_with_invitation
                                    .value()
                                    .get()
                                    .map(|res| {
                                        match res {
                                            Err(ServerFnError::ServerError(err)) => {
                                                view! {
                                                    <p class="text-error mb-2 text-xs italic">{err}</p>
                                                }
                                                    .into_any()
                                            }
                                            _ => ().into_any(),
                                        }
                                    })
                            }}
                            <h2 class="mb-2 text-[12px] leading-[16px] font-bold">INVITE LINK</h2>
                            <input
                                type="text"
                                name="invitation"
                                class="input rounded input-secondary bg-secondary h-[40px] text-[14px] w-full"
                                placeholder="https://discord.gg/hTKzmak"
                            />
                        </div>
                    </div>
                    <div class="mb-4">
                        <h2 class="mb-2 text-[12px] leading-[16px] font-bold">
                            INVITE SHOULD LOOK LIKE
                        </h2>
                        <div class="text-[14px] leading-[18px]">hTKzmak</div>
                        <div class="text-[14px] leading-[18px]">"https://discord.gg/hTKzmak"</div>
                        <div class="text-[14px] leading-[18px]">
                            "https://discord.gg/cool-people"
                        </div>
                    </div>
                    <A
                        on:click=move |_| use_create_server.is_open.set(false)
                        href="search_servers"
                        {..}
                        class="rounded-lg bg-base-300/30 hover:bg-base-content/30 mb-4 flex items-center p-3 text-left"
                    >
                        <div class="mr-3 w-[40px] h-[40px] bg-primary rounded-full flex items-center justify-center">
                            // <Icon icon=icondata::RiCompassMapLine />
                        </div>
                        <div>
                            <h2 class="text-[16px] leading-[20px] font-bold">
                                "Don't have an invite?"
                            </h2>
                            <div class="text-[12px] leading-[16px] font-normal">
                                Check out Discoverable communities in Server Discovery.
                            </div>
                        </div>
                        // <Icon icon=icondata::RiArrowRightSArrowsLine />
                    </A>
                </div>
                <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-300/60">
                    <SlideBack
                        attr:r#type="reset"
                        class="w-auto min-h-min h-[38px] py-0.5 px-1 leading-[16px] hover:underline text-base-content text-[14px]"
                    >
                        Back
                    </SlideBack>
                    <button
                        type="submit"
                        class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation"
                        disabled=move || join_with_invitation.pending().get()
                    >
                        Join Server
                    </button>
                </div>
            </ActionForm>
        </Transition>
    }
}
