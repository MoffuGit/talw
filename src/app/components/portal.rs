use leptos::*;

#[component]
pub fn ProvidePortalContext(children: Children, name: &'static str) -> impl IntoView {
    let signal = create_rw_signal(false);
    create_effect(move |_| log::info!("change on provide portal {}: {}", name, signal.get()));

    provide_context(signal);
    children()
}

#[component]
pub fn PortalTrigger() -> impl IntoView {
    let signal = use_context::<RwSignal<bool>>().expect("signal");

    view! {
        <button class="w-10 h-5 bg-red-500" on:click=move |_| signal.update(|value| *value = true)/>
    }
}

#[component]
pub fn ClosePortal() -> impl IntoView {
    let signal = use_context::<RwSignal<bool>>().expect("signal");
    view! {
        <button class="w-10 h-5 bg-red-500" on:click=move |_| signal.update(|value| *value=false)/>
    }
}

#[component]
pub fn PortalContent(children: ChildrenFn, class: &'static str) -> impl IntoView {
    let signal = use_context::<RwSignal<bool>>().expect("signal");

    let show = create_rw_signal(false);
    create_effect(move |_| show.update(|value| *value = true));

    view! {
        <Show when=move || show.get() && signal.get()>
            <Portal mount=document().get_element_by_id("app").unwrap() clone:children>
                <div class=class>
                    {children.clone()}
                </div>
            </Portal>
        </Show>
    }
}
