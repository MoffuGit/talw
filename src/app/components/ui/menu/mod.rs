use leptos::*;
use leptos_use::on_click_outside;

#[derive(Clone)]
struct MenuProviderContext {
    open: RwSignal<bool>,
    modal: bool,
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
}

#[component]
pub fn ProvideMenu(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] trigger_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] content_ref: Option<NodeRef<html::Div>>,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));
    let trigger_ref = trigger_ref.unwrap_or(create_node_ref::<html::Div>());
    let content_ref = content_ref.unwrap_or(create_node_ref::<html::Div>());
    provide_context(MenuProviderContext {
        open,
        modal,
        trigger_ref,
        content_ref,
    });
    children()
}

#[component]
pub fn MenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let trigger_ref = context.trigger_ref;
    view! {
        <div class=class on:click=move |_| open.set(true) node_ref=trigger_ref>
            {children.map(|children| children())}
        </div>
    }
}

#[component]
pub fn MenuContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] style: Option<Signal<String>>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let content_ref = context.content_ref;
    let style = style.unwrap_or_default();
    let visibility = move || {
        if !context.open.get() {
            "visibility: hidden;"
        } else {
            ""
        }
    };
    create_effect(move |_| {
        if context.modal {
            if let Some(app) = document().get_element_by_id("app") {
                if context.open.get() {
                    let _ = app.class_list().add_1("pointer-events-none");
                } else {
                    let _ = app.class_list().remove_1("pointer-events-none");
                }
            }
        }
    });
    let _ = on_click_outside(content_ref, move |_| context.open.set(false));
    view! {
        <Provider value=context.clone() clone:children>
            <Portal mount=document().get_element_by_id("app").unwrap()>
                <div style= move || format!("{} {}", visibility(), style.get()) class=&class node_ref=content_ref>
                    {children.clone()}
                </div>
            </Portal>
        </Provider>
    }
}
