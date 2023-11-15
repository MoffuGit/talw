pub mod create_server;
pub mod slide_modal;

use leptos::{html::Dialog, logging::warn, *};

pub type ModalProviderContext = RwSignal<bool>;

#[component]
pub fn ModalProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
) -> impl IntoView {
    let is_open = if let Some(signal) = open {
        signal
    } else {
        create_rw_signal(false)
    };
    provide_context(is_open);
    view! {
        {children()}
    }
}

#[component]
pub fn ModalTrigger(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>().expect("have the context");

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
) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>().expect("have this context");
    view! {
        <button on:click=move |_| is_open.update(|value| *value = false) class=class>
            {children.map(|children| children())}
        </button>
    }
}

#[component]
pub fn ModalPortal(children: ChildrenFn) -> impl IntoView {
    let (show, set_show) = create_signal(false);
    create_effect(move |_| set_show(true));
    let children = store_value(children);
    view! {
        <Show when=move || show.get()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
                {children()}
            </Portal>
        </Show>
    }
}

#[component]
pub fn ModalContent(children: Children, class: &'static str) -> impl IntoView {
    let is_open = use_context::<ModalProviderContext>().expect("have context");

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
        <dialog class="modal" _ref=dialog_ref>
            <div class=format!("modal-box {}", class)>
                {children()}
            </div>
            <form method="dialog" class="modal-backdrop">
                <button on:click=move |_| is_open.update(|value| *value = false)/>
            </form>
        </dialog>
    }
}
