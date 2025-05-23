use crate::app::api::channel::{use_channel, UpdateChannel};
use crate::app::components::ui::modal::ModalProvider;
use crate::app::components::ui::modal::*;
use crate::entities::channel::{Channel, ChannelStoreFields};
use leptos::{html, prelude::*};
//use leptos_icons::Icon;

use leptos::ev::MouseEvent;
use reactive_stores::Field;

#[component]
pub fn EditChannelModal(
    #[prop(into)] channel: Field<Channel>,
    class: &'static str,
    #[prop(optional)] on_click: Signal<()>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <ModalProvider content_ref=content_ref open=open>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div class="text-start p-[16px] w-full">
                    <h1 class="font-bold text-[24px] leading-[30px]">"Edit Channel"</h1>
                    <ModalClose class="absolute right-2 top-2 flex items-center group bg-none">
                        // <Icon icon=icondata::RiCloseSystemLine />
                    // class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"
                    </ModalClose>
                </div>
                <EditChannelModalContent
                    channel=channel
                    open=open
                />
            </ModalContent>
        </ModalProvider>
    }
}

#[component]
fn EditChannelModalContent(
    open: RwSignal<bool>,
    #[prop(into)] channel: Field<Channel>,
) -> impl IntoView {
    let update_channel = use_channel().update_channel;
    let new_name = RwSignal::new(channel.name().get());
    let new_topic = RwSignal::new(channel.topic().get());
    Effect::new(move |_| {
        update_channel.version().with(|_| {
            if let Some(Ok(_)) = update_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });
    Effect::new(move |_| {
        if !open.get() {
            new_topic.set(channel.topic().get());
            new_name.set(channel.name().get());
        }
    });
    let on_click = move |evt: MouseEvent| {
        evt.stop_propagation();
        let topic = channel.topic().get();
        let name = channel.name().get();
        if new_name.get() == name && new_topic.get() == topic {
            return;
        }
        update_channel.dispatch(UpdateChannel {
            name: new_name.with(|new_name| {
                if new_name != &name {
                    Some(new_name)
                } else {
                    None
                }
                .cloned()
            }),
            topic: new_topic.with(|new_topic| {
                if new_topic != &topic {
                    new_topic.clone()
                } else {
                    None
                }
            }),
            server_id: channel.server_id().get(),
            channel_id: channel.id().get(),
        });
    };
    view! {
            <div class="px-[16px] w-full">
                <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">
                    "channel name"
                </div>
                <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                    <input
                        type="text"
                        prop:value=move || new_name.get()
                        on:input=move |evt| {
                            new_name.set(event_target_value(&evt))
                        }
                        class="w-full h-10 bg-base-300 py-[10px]"
                    />
                </div>
                <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">
                    "channel topic"
                </div>
                <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                    <input
                        type="text"
                        prop:value=move || new_topic.get().unwrap_or_default()
                        on:input=move |evt| {
                            new_topic.set(Some(event_target_value(&evt)))
                        }
                        class="w-full h-10 bg-base-300 py-[10px]"
                    />
                </div>
            </div>

            <div class="relative p-4 flex justify-end w-full bg-base-200">
                <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                    "Cancel"
                </ModalClose>
                <button
                    on:click=on_click
                    class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-secondary text-seconday-content"
                    disabled=move || update_channel.pending().get()
                >
                    "Rename Channel"
                </button>
            </div>

    }
}
