pub mod slide_modal;

use leptos::{
    context::Provider,
    html::{self, Dialog, Div},
    logging::warn,
    portal::Portal,
    prelude::*,
};

#[derive(Clone)]
pub struct ModalProviderContext {
    open: RwSignal<bool>,
    on_close: Option<Signal<()>>,
    content_ref: NodeRef<Div>,
    dialog_ref: NodeRef<Dialog>,
    trigger_ref: NodeRef<Div>,
}

#[component]
pub fn ModalProvider(
    children: Children,
    #[prop(optional, into)] on_close: Option<Signal<()>>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] trigger_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] content_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] dialog_ref: Option<NodeRef<html::Dialog>>,
) -> impl IntoView {
    let open = open.unwrap_or(RwSignal::new(false));
    let trigger_ref = trigger_ref.unwrap_or_default();
    let content_ref = content_ref.unwrap_or_default();
    let dialog_ref = dialog_ref.unwrap_or_default();

    view! {
        <Provider value=ModalProviderContext {
            open,
            on_close,
            trigger_ref,
            content_ref,
            dialog_ref,
        }>{children()}</Provider>
    }
}

#[component]
pub fn ModalTrigger(
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] on_click: Option<Signal<()>>,
) -> impl IntoView {
    let context = use_context::<ModalProviderContext>().expect("have the context");
    let is_open = context.open;
    let trigger_ref = context.trigger_ref;

    view! {
        <div
            on:click=move |_| {
                is_open.set(true);
                if let Some(on_click) = on_click {
                    on_click.get();
                }
            }
            class=class
            node_ref=trigger_ref
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ModalClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] on_click: Option<Signal<()>>,
) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>()
        .expect("have this context")
        .open;
    view! {
        <button
            on:click=move |_| {
                if let Some(on_click) = on_click {
                    on_click.get()
                }
                is_open.set(false);
            }
            class=class
        >
            {children.map(|children| children())}
        </button>
    }
}

#[component]
pub fn ModalContent(children: ChildrenFn, class: &'static str) -> impl IntoView {
    let modal_context = use_context::<ModalProviderContext>().expect("have context");
    let on_close = modal_context.on_close;
    let is_open = modal_context.open;
    let dialog_ref = modal_context.dialog_ref;
    let content_ref = modal_context.content_ref;
    Effect::new(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if is_open.get() {
                if dialog.show_modal().is_err() {
                    warn!("<Modal/> error while calling HTMLDialogElement.showModal()");
                    dialog.set_open(true);
                }
            } else {
                dialog.close();
            }
        }
    });

    let show = RwSignal::new(false);
    let children = StoredValue::new(children);
    Effect::new(move |_| show.update(|value| *value = true));
    view! {
        <Show when=move || show.get()>
            <Portal mount=document().get_element_by_id("app").expect("acces to the app")>
                <dialog
                    class="modal"
                    node_ref=dialog_ref
                    on:close=move |_| {
                        if let Some(on_close) = on_close {
                            on_close.get()
                        }
                    }
                >
                    <div class=format!("modal-box {}", class) node_ref=content_ref>
                        {children.get_value()()}
                    </div>
                    <form method="dialog" class="modal-backdrop">
                        <button on:click=move |_| is_open.set(false) />
                    </form>
                </dialog>
            </Portal>
        </Show>
    }
}
