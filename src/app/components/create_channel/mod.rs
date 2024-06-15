use crate::app::api::server::use_server;
use crate::app::components::ui::modal::*;
use crate::entities::channel::ChannelType;
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::ActionForm;
use uuid::Uuid;

#[component]
pub fn CreateChannelModal(
    class: &'static str,
    on_click: Signal<()>,
    server_id: Uuid,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let create_channel = use_server().create_channel;
    let channel_type = create_rw_signal::<ChannelType>(ChannelType::TEXT);
    let form_ref = create_node_ref::<html::Form>();
    let on_close = move || {
        if let Some(form) = form_ref.get() {
            form.reset();
        }
        channel_type.set(ChannelType::TEXT);
    };
    create_effect(move |_| {
        create_channel.version().with(|_| {
            if let Some(Ok(_)) = create_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });
    view! {
        <ModalProvider open=open on_close=Signal::derive(on_close)>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div class="text-start p-[16px] w-full">
                    <h1 class="font-bold text-[24px] leading-[30px]">"Create Channel"</h1>
                    <ModalClose /* attr:type="reset" */ class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <ActionForm action=create_channel node_ref=form_ref class="w-full">
                    <div class="px-[16px] w-full">
                        <div class="mb-5">
                            <div class="text-[12px] mb-2 leading-[18px] uppercase font-bold text-base-content/80">"channel type"</div>
                            <div class="mb-2 rounded bg-base-200 flex justify-between w-full py-[10px] px-3 items-center" on:click=move |_| channel_type.set(ChannelType::TEXT)>
                                <Icon icon=icondata::RiHashtagEditor class="w-6 h-6 mr-3"/>
                                <div class="flex flex-col mr-2">
                                    <div class="font-medium">"Text"</div>
                                    <div class="text-xs">"Send messages, images, GIFs, emoji, opinions, and puns"</div>
                                </div>
                                // <div class=move || format!("rounded-full w-6 h-6 border-base-content border border-opacity-20 appearance-none bg-base-100 cursor-pointer {}", match {})/>
                                <input type="radio" name="channel_type" class="radio" checked=move || channel_type.get() == ChannelType::TEXT value=ChannelType::TEXT/>
                            </div>
                            <div class="mb-2 rounded bg-base-200 flex justify-between py-[10px] px-3 items-center" on:click=move |_| channel_type.set(ChannelType::VOICE)>
                                <Icon icon=icondata::RiVolumeUpMediaFill class="w-6 h-6 mr-3"/>
                                <div class="flex flex-col mr-2">
                                    <div class="font-medium">"Voice"</div>
                                    <div class="text-xs">"Hang out together with voice, video and screen share"</div>
                                </div>
                                <input type="radio" name="channel_type" class="radio" checked=move || channel_type.get() == ChannelType::VOICE value=ChannelType::VOICE/>
                            </div>
                        </div>
                        <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">"channel name"</div>
                        <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                            {
                                move || {
                                    match channel_type.get() {
                                        ChannelType::TEXT => view!{<Icon icon=icondata::RiHashtagEditor class="w-5 h-5 mx-2"/>},
                                        ChannelType::VOICE => view!{<Icon icon=icondata::RiVolumeUpMediaFill class="w-5 h-5 mx-2"/>},
                                        ChannelType::ANNOUNCEMENTS => view!{<Icon icon=icondata::RiHashtagEditor class="w-5 h-5 mx-2"/>},
                                        ChannelType::RULES => view!{<Icon icon=icondata::RiHashtagEditor class="w-5 h-5 mx-2"/>},
                                    }
                                }
                            }
                            <input name="name" minlength="1" type="text" placeholder="new-channel" class="w-full h-10 bg-base-300 py-[10px]"/>
                        </div>
                    </div>
                    <div class="relative p-4 flex justify-end w-full bg-base-200">
                        <ModalClose attr:type="reset" class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                            "Cancel"
                        </ModalClose>
                        <input value=server_id.to_string() type="hidden" name="server_id"/>
                        <button type="submit" class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-secondary text-seconday-content" >
                            "Create Channel"
                        </button>
                    </div>
                </ActionForm>
            </ModalContent>
        </ModalProvider>
    }
}
