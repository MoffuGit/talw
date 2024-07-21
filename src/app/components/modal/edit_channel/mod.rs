use crate::app::api::channel::use_channel;
use crate::app::components::ui::modal::ModalProvider;
use crate::app::components::ui::modal::*;
use crate::entities::channel::Channel;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::ActionForm;

#[allow(non_snake_case)]
#[component]
pub fn EditChannelModal(
    channel: Channel,
    class: &'static str,
    #[prop(optional)] on_click: Option<Signal<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let form_ref = create_node_ref::<html::Form>();
    let rename_channel = use_channel().rename_channel;
    let on_close = move || {
        if let Some(form) = form_ref.get() {
            form.reset();
        }
    };
    create_effect(move |_| {
        rename_channel.version().with(|_| {
            if let Some(Ok(_)) = rename_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });
    let channel = store_value(channel);
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
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div class="text-start p-[16px] w-full">
                    <h1 class="font-bold text-[24px] leading-[30px]">"Edit Channel"</h1>
                    <ModalClose /* attr:type="reset" */ class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <ActionForm action=rename_channel node_ref=form_ref class="w-full">
                    <div class="px-[16px] w-full">
                        <div class="text-[12px] mb-0.5 leading-[18px] uppercase font-bold text-base-content">"channel name"</div>
                        <div class="mt-2 mb-4 w-full bg-base-300 rounded flex items-center">
                            <input name="new_name" minlength="1" type="text" value=channel.get_value().name class="w-full h-10 bg-base-300 py-[10px]"/>
                        </div>
                    </div>
                    <div class="relative p-4 flex justify-end w-full bg-base-200">
                        <ModalClose attr:type="reset" class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                            "Cancel"
                        </ModalClose>
                        <input value=channel.get_value().server_id.to_string() type="hidden" name="server_id"/>
                        <input value=channel.get_value().id.to_string() type="hidden" name="channel_id"/>
                        <button type="submit" class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-secondary text-seconday-content" disabled=move || rename_channel.pending().get()>
                            "Rename Channel"
                        </button>
                    </div>
                </ActionForm>
            </ModalContent>
        </ModalProvider>
    }
}
