use crate::app::api::channel::{get_channel_topic, use_channel, UpdateChannel};
use crate::app::components::ui::modal::ModalProvider;
use crate::app::components::ui::modal::*;
use crate::entities::channel::Channel;
use leptos::*;
use leptos_icons::Icon;

#[allow(non_snake_case)]
#[component]
pub fn EditChannelModal(
    channel: Channel,
    class: &'static str,
    #[prop(optional)] on_click: Option<Signal<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = create_rw_signal(false);

    let Channel { name, .. } = channel;
    let name = store_value(name);

    let use_channel = use_channel();
    let update_channel = use_channel.update_channel;

    let new_name = create_rw_signal(name.get_value());
    let new_topic = create_rw_signal(String::default());

    let current_topic = create_resource(
        move || (update_channel.version().get()),
        move |_| get_channel_topic(channel.id),
    );

    create_effect(move |_| {
        update_channel.version().with(|_| {
            if let Some(Ok(_)) = update_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });

    let on_close = move || {
        new_name.set(name.get_value());
        new_topic.set(String::default());
    };
    view! {
        <ModalProvider open=open on_close=Signal::derive(on_close)>
            {
                if let Some(on_click) = on_click {
                    view! {
                        <ModalTrigger class=class on_click=on_click>
                            {children.map(|children| children())}
                        </ModalTrigger>
                    }.into_view()
                } else {
                    view! {
                        <ModalTrigger class=class>
                            {children.map(|children| children())}
                        </ModalTrigger>
                    }.into_view()
                }
            }
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center" >

                <div class="text-start p-[16px] w-full">
                    <h1 class="font-bold text-[24px] leading-[30px]">"Edit Channel"</h1>
                    <ModalClose /* attr:type="reset" */ class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <Transition fallback=move || ()>
                    {
                        move || {
                            match current_topic.get() {
                                Some(Ok(topic)) => {
                                    let topic = store_value(topic.unwrap_or_default());
                                    new_topic.set(topic.get_value());
                                    view!{
                                        <div class="px-[16px] w-full">
                                            <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">"channel name"</div>
                                            <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                                                <input type="text" prop:value=new_name on:input=move |evt| {new_name.set(event_target_value(&evt))} class="w-full h-10 bg-base-300 py-[10px]"/>
                                            </div>
                                                <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">"channel topic"</div>
                                                <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                                                    <input type="text" prop:value=new_topic on:input=move |evt| {new_topic.set(event_target_value(&evt))} class="w-full h-10 bg-base-300 py-[10px]"/>
                                                </div>
                                        </div>

                                        <div class="relative p-4 flex justify-end w-full bg-base-200">
                                            <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                                                "Cancel"
                                            </ModalClose>
                                            <button on:click=move |evt| {
                                                evt.stop_propagation();
                                                match (new_name.get() != name.get_value(), new_topic.get() != topic.get_value()) {
                                                    (true, true) =>  {
                                                        update_channel.dispatch(UpdateChannel {
                                            channel_id: channel.id,
                                            name: Some(new_name.get()),
                                            topic: Some(new_topic.get()),
                                                        });
                                                    },
                                                    (true, false) => {
                                                        update_channel.dispatch(UpdateChannel {
                                            channel_id: channel.id,
                                            name: Some(new_name.get()),
                                            topic: None,
                                                        });

                                                    },
                                                    (false, true) => {
                                                        update_channel.dispatch(UpdateChannel {
                                            channel_id: channel.id,
                                            name: None,
                                            topic: Some(new_topic.get()),
                                                        });

                                                    }
                                                    _ => ()
                                                }
                                            } class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-secondary text-seconday-content" disabled=move || update_channel.pending().get()>
                                                "Rename Channel"
                                            </button>
                                        </div>

                                    }.into_view()
                                },
                                _ => ().into_view()
                            }
                        }
                    }
                </Transition>
            </ModalContent>
        </ModalProvider>
    }
}
