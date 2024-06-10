use leptos::*;

#[derive(Clone)]
pub struct ContextPortal(RwSignal<bool>, String);

#[component]
pub fn ProvidePortalContext(children: Children, name: &'static str) -> impl IntoView {
    let signal = create_rw_signal(false);
    create_effect(move |_| log::info!("change on provide portal {}: {}", name, signal.get()));

    provide_context(ContextPortal(signal, name.to_string()));
    children()
}

#[component]
pub fn PortalTrigger(class: &'static str) -> impl IntoView {
    let signal = use_context::<ContextPortal>().expect("signal").0;

    view! {
        <button class=format!("w-10 h-5 {}", class) on:click=move |_| signal.update(|value| *value = true)/>
    }
}

#[component]
pub fn ClosePortal(#[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<ContextPortal>().expect("signal");
    view! {
        <button class=format!("w-10 h-5 {}", class) on:click=move |_| context.0.update(|value| *value=false)>
            {context.1}
        </button>
    }
}

#[component]
pub fn PortalContent(children: ChildrenFn, #[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<ContextPortal>().expect("signal");
    let signal = context.0;

    view! {
        <Show when=move || signal.get()>
            <Provider value=context.clone() clone:children>
            <Portal mount=document().get_element_by_id("app").expect("acces to the app") >
                <div class=class>
                     {children.clone()}
                </div>
            </Portal>
            </Provider>
        </Show>
    }
}
