use leptos::context::Provider;
use leptos::ev::contextmenu;
use leptos::portal::Portal;
use leptos::{html, logging, prelude::*};
use leptos_use::{
    on_click_outside_with_options, use_document, use_event_listener,
    use_event_listener_with_options, OnClickOutsideOptions, UseEventListenerOptions,
};

#[derive(Clone)]
struct MenuProviderContext {
    open: RwSignal<bool>,
    hidden: RwSignal<bool>,
    modal: bool,
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    trigger_key: TriggerKey,
}

#[derive(Clone, PartialEq, Copy)]
pub enum TriggerKey {
    Ltr,
    Rtl,
}

#[allow(non_snake_case)]
#[component]
pub fn MenuProvider(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] hidden: Option<RwSignal<bool>>,
    #[prop(optional)] trigger_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] content_ref: Option<NodeRef<html::Div>>,
    trigger_key: TriggerKey,
) -> impl IntoView {
    let open = open.unwrap_or(RwSignal::new(false));
    let hidden = hidden.unwrap_or(RwSignal::new(false));
    let trigger_ref = trigger_ref.unwrap_or(NodeRef::<html::Div>::new());
    let content_ref = content_ref.unwrap_or(NodeRef::<html::Div>::new());
    view! {
        <Provider value=MenuProviderContext {
            open,
            modal,
            hidden,
            trigger_ref,
            content_ref,
            trigger_key,
        }>{children()}</Provider>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MenuTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let open = context.open;
    let hidden = context.hidden;
    let trigger_ref = context.trigger_ref;
    match context.trigger_key {
        TriggerKey::Ltr => view! {
            <div
                class=move || {
                    format!(
                        "{} {}",
                        class,
                        match open.get() {
                            true => "pointer-events-none",
                            false => "",
                        },
                    )
                }
                on:click=move |_| {
                    open.set(true);
                    hidden.set(false);
                }
                node_ref=trigger_ref
            >
                {children.map(|children| children())}
            </div>
        }
        .into_any(),
        TriggerKey::Rtl => view! {
            <div
                class=move || {
                    format!(
                        "{} {}",
                        class,
                        match open.get() {
                            true => "pointer-events-auto",
                            false => "",
                        },
                    )
                }
                on:contextmenu=move |evt| {
                    evt.prevent_default();
                    open.set(true);
                    hidden.set(false);
                }
                node_ref=trigger_ref
            >
                {children.map(|children| children())}
            </div>
        }
        .into_any(),
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MenuContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] style: Option<Signal<String>>,
    #[prop(optional)] ignore: Option<Vec<NodeRef<html::Div>>>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let content_ref = context.content_ref;
    let style = style.unwrap_or_default();
    Effect::new(move |_| {
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

    let on_click_outside_options = if let Some(ignore) = ignore {
        OnClickOutsideOptions::default().ignore(ignore)
    } else {
        OnClickOutsideOptions::default()
    };
    let _ = on_click_outside_with_options(
        content_ref,
        move |_| {
            if context.open.get() {
                context.open.set(false)
            }
        },
        on_click_outside_options,
    );
    let show = RwSignal::new(false);
    Effect::new(move |_| {
        show.set(true);
    });
    let class = StoredValue::new(class);
    view! {
        <Provider value=context.clone()>
            <Show when=move || {
                show.get() && context.open.get()
            }>
                {if context.trigger_key == TriggerKey::Rtl {
                    let _ = use_event_listener_with_options(
                        use_document(),
                        contextmenu,
                        move |_| {
                            if context.open.get() {
                                context.open.set(false)
                            }
                        },
                        UseEventListenerOptions::default().capture(true)
                    );
                }}
                <Portal
                    mount=document().get_element_by_id("app").expect("acces to app")
                    clone:children
                >
                    <div
                        style=move || if context.open.get() { style.get() } else { "".to_string() }
                        class=move || {
                            format!(
                                "{} {}",
                                class.get_value(),
                                match context.hidden.get() {
                                    true => "hidden",
                                    false => "",
                                },
                            )
                        }
                        node_ref=content_ref
                    >
                        {children.clone().map(|children| children())}
                    </div>
                </Portal>
            </Show>
        </Provider>
    }
}
