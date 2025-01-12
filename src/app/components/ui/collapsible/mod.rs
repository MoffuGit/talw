use leptos::*;

#[derive(Clone)]
pub struct CollapsibleProviderContext {
    is_open: RwSignal<bool>,
}

#[component]
pub fn CollapsibleProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
) -> impl IntoView {
    let is_open = open.unwrap_or(create_rw_signal(false));

    provide_context(CollapsibleProviderContext { is_open });

    children()
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<CollapsibleProviderContext>().expect("have collapsible context");
    let is_open = context.is_open;

    view! {
        <div class=class on:click=move |_| is_open.update(|is_open| *is_open = !*is_open)>
            {children.map(|children| children())}
        </div>
    }
}

#[component]
pub fn CollapsibleContent(children: ChildrenFn) -> impl IntoView {
    let context = use_context::<CollapsibleProviderContext>().expect("have collapsible context");
    let is_open = context.is_open;

    view! { <Show when=move || is_open.get()>{children()}</Show> }
}
