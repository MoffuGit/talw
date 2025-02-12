use leptos::context::Provider;
use leptos::portal::Portal;
use leptos::prelude::*;

#[derive(Clone)]
pub struct ContextPortal(RwSignal<bool>, String);

#[component]
pub fn ProvidePortalContext(children: Children, name: &'static str) -> impl IntoView {
    let signal = RwSignal::new(false);

    provide_context(ContextPortal(signal, name.to_string()));
    children()
}

#[component]
pub fn PortalTrigger(class: &'static str) -> impl IntoView {
    let signal = use_context::<ContextPortal>().expect("signal").0;

    view! {
        <button
            class=format!("w-10 h-5 {}", class)
            on:click=move |_| signal.update(|value| *value = true)
        />
    }
}

#[component]
pub fn ClosePortal(#[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<ContextPortal>().expect("signal");
    view! {
        <button
            class=format!("w-10 h-5 {}", class)
            on:click=move |_| context.0.update(|value| *value = false)
        >
            {context.1}
        </button>
    }
}

#[component]
pub fn PortalContent(children: ChildrenFn, #[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<ContextPortal>().expect("signal");
    let signal = context.0;
    let children = StoredValue::new(children);

    view! {
        <Show when=move || signal.get()>
            <Provider value=context.clone()>
                <Portal mount=document().get_element_by_id("app").expect("acces to the app")>
                    <div class=class>{children.get_value()()}</div>
                </Portal>
            </Provider>
        </Show>
    }
}
