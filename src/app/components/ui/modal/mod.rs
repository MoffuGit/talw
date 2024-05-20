pub mod slide_modal;

use leptos::{html::Dialog, logging::warn, *};

#[derive(Debug, Clone)]
pub struct ModalProviderContext {
    pub open: RwSignal<bool>,
    on_close: Option<Signal<()>>,
}

#[component]
pub fn ModalProvider(
    children: Children,
    #[prop(optional, into)] on_close: Option<Signal<()>>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));

    provide_context(ModalProviderContext { open, on_close });

    children()
}

#[component]
pub fn ModalTrigger(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>()
        .expect("have the context")
        .open;

    view! {
        <div on:click=move |_| is_open.update(|value| *value = !*value) class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] on_click: Option<Signal<()>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>()
        .expect("have this context")
        .open;
    view! {
        <button
            {..attrs}
            on:click=move |_| {
                if let Some(on_click) = on_click { on_click.get() }
                is_open.update(|value| *value = false);
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
    let dialog_ref = create_node_ref::<Dialog>();

    create_effect(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if is_open.get() {
                if dialog.show_modal().is_err() {
                    warn!("<Modal/> error while calling HTMLDialogElement.showModal()");
                    dialog.set_open(true);
                }
            } else {
                dialog.close();
            }
        } else {
            warn!("cant get the dialog ref")
        }
    });

    view! {
        <dialog class="modal" _ref=dialog_ref on:close=move |_| {
            if let Some(on_close) = on_close { on_close.get() }
        }>
            <div class=format!("modal-box {}", class)>
                {children}
            </div>
            <form method="dialog" class="modal-backdrop">
                <button on:click=move |_| is_open.update(|value| *value = false)/>
            </form>
        </dialog>
    }
}
