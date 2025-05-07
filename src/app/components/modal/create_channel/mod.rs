use crate::app::api::channel::use_channel;
use crate::app::components::ui::modal::*;
use crate::entities::channel::ChannelType;
// use icondata::{self, Icon};
use leptos::either::Either;
use leptos::{html, prelude::*};
//use leptos_icons::*;

use uuid::Uuid;

#[component]
pub fn CreateChannelModal(
    class: &'static str,
    #[prop(optional)] on_click: Option<Signal<()>>,
    #[prop(into)] server_id: Signal<Uuid>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] category_id: Option<Signal<Uuid>>,
    #[prop(optional, into)] category_name: Option<Signal<String>>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let channel_type = RwSignal::new(ChannelType::TEXT);
    let form_ref = NodeRef::<html::Form>::new();
    let on_close = move || {
        if let Some(form) = form_ref.get() {
            form.reset();
        }
        channel_type.set(ChannelType::TEXT);
    };
    let create_channel = use_channel().create_channel;
    Effect::new(move |_| {
        create_channel.version().with(|_| {
            if let Some(Ok(_)) = create_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });
    view! {
        <ModalProvider content_ref=content_ref open=open on_close=Signal::derive(on_close)>
            {if let Some(on_click) = on_click {
                Either::Left(
                    view! {
                        <ModalTrigger class=class on_click=on_click>
                            {children.map(|children| children())}
                        </ModalTrigger>
                    },
                )
            } else {
                Either::Right(
                    view! {
                        <ModalTrigger class=class>
                            {children.map(|children| children())}
                        </ModalTrigger>
                    },
                )
            }}
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div class="text-start p-[16px] w-full">
                    <h1 class="font-bold text-[24px] leading-[30px]">"Create Channel"</h1>
                    {
                        move || {
                            category_name.get().map(|name| {
                                view!{
                                    <p class="leading-[30px] text-xs">{format!("in {name}")}</p>
                                }
                            })
                        }
                    }
                    <ModalClose class="absolute right-2 top-2 flex items-center group bg-none">
                        <div/>
                        // <Icon icon=icondata::RiCloseSystemLine />
                    // class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"
                    </ModalClose>
                </div>
                <ActionForm action=create_channel node_ref=form_ref>
                    // {..}
                    // class="w-full"
                    <div class="px-[16px] w-full">
                        <div class="mb-5">
                            <div class="text-[12px] mb-2 leading-[18px] uppercase font-bold text-base-content/80">
                                "channel type"
                            </div>
                            <div
                                class="mb-2 rounded bg-base-200 flex justify-between w-full py-[10px] px-3 items-center"
                                on:click=move |_| channel_type.set(ChannelType::TEXT)
                            >
                                // <Icon icon=icondata::RiHashtagEditor />
                                <div class="flex flex-col mr-2">
                                    <div class="font-medium">"Text"</div>
                                    <div class="text-xs">
                                        "Send messages, images, GIFs, emoji, opinions, and puns"
                                    </div>
                                </div>
                            // <div class=move || format!("rounded-full w-6 h-6 border-base-content border border-opacity-20 appearance-none bg-base-100 cursor-pointer {}", match {})/>
                            <input
                                type="radio"
                                name="channel_type"
                                class="radio"
                                checked=move || channel_type.get() == ChannelType::TEXT
                                value=String::from(ChannelType::TEXT)
                            />
                            </div>
                            <div
                                class="mb-2 rounded bg-base-200 flex justify-between py-[10px] px-3 items-center"
                                on:click=move |_| channel_type.set(ChannelType::VOICE)
                            >
                                // <Icon icon=icondata::RiVolumeUpMediaFill />
                                // class="w-6 h-6 mr-3"
                                <div class="flex flex-col mr-2">
                                    <div class="font-medium">"Voice"</div>
                                    <div class="text-xs">
                                        "Hang out together with voice, video and screen share"
                                    </div>
                                </div>
                                <input
                                type="radio"
                                name="channel_type"
                                class="radio"
                                checked=move || channel_type.get() == ChannelType::VOICE
                                value=String::from(ChannelType::VOICE)
                                />
                            </div>
                        </div>
                        <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">
                            "channel name"
                        </div>
                        <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                            // {move || {
                            //     view! { <Icon icon=Icon::from(channel_type.get()) /> }
                            // }}
                            <input
                                name="name"
                                minlength="1"
                                type="text"
                                placeholder="new-channel"
                                class="w-full h-10 bg-base-300 py-[10px]"
                            />
                        </div>
                    </div>
                    <div class="relative p-4 flex justify-end w-full bg-base-200">
                        <ModalClose
                            attr:r#type="reset"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline"
                        >
                            "Cancel"
                        </ModalClose>
                        <input value=move || server_id.get().to_string() type="hidden" name="server_id" />
                        {
                            category_id.map(|id| {
                                view!{
                                    <input
                                        value=move || id.get().to_string()
                                        type="hidden"
                                        name="category_id"
                                    />
                                }
                            })
                        }
                        <button
                            type="submit"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-secondary text-seconday-content"
                            disabled=move || create_channel.pending().get()
                        >
                            "Create Channel"
                        </button>
                    </div>
                </ActionForm>
            </ModalContent>
        </ModalProvider>
    }
}
