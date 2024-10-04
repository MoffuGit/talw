use leptos::ev::contextmenu;
use leptos::*;
use leptos_use::{on_click_outside, use_document, use_event_listener};

#[derive(Clone)]
struct MenuProviderContext {
    open: RwSignal<bool>,
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
    #[prop(optional)] trigger_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] content_ref: Option<NodeRef<html::Div>>,
    trigger_key: TriggerKey,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));
    let trigger_ref = trigger_ref.unwrap_or(create_node_ref::<html::Div>());
    let content_ref = content_ref.unwrap_or(create_node_ref::<html::Div>());
    view! {
        <Provider value=MenuProviderContext{ open, modal, trigger_ref, content_ref, trigger_key}>
            {   children()}
        </Provider>
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
    let trigger_ref = context.trigger_ref;
    match context.trigger_key {
        TriggerKey::Ltr => view! {
            <div
                class=move || {
                    format!("{} {}", class, match open.get() {
                        true => "pointer-events-none",
                        false => ""
                    })
                }
                on:click=move |_| {
                    open.set(true);
                }
                node_ref=trigger_ref>
                {children.map(|children| children())}
            </div>
        },
        TriggerKey::Rtl => view! {
        <div class=move || {
            format!("{} {}", class, match open.get() {
                            true => "pointer-events-auto",
                            false => ""
                            })
            }
            on:contextmenu=move |evt| {
                evt.prevent_default();
                open.set(true);
            }
            node_ref=trigger_ref>
                {children.map(|children| children())}
        </div>
        },
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MenuContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] style: Option<Signal<String>>,
) -> impl IntoView {
    let context = use_context::<MenuProviderContext>().expect("acces to menu context");
    let content_ref = context.content_ref;
    let style = style.unwrap_or_default();
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
    let _ = on_click_outside(content_ref, move |_| {
        if context.open.get() {
            context.open.set(false)
        }
    });
    if context.trigger_key == TriggerKey::Rtl {
        let _ = use_event_listener(use_document(), contextmenu, move |_| {
            if context.open.get() {
                context.open.set(false)
            }
        });
    }
    let show = create_rw_signal(false);
    create_effect(move |_| {
        show.set(true);
    });
    let class = store_value(class);
    view! {
        <Provider value=context.clone() clone:children>
            <Show when=move || show.get()>
                <Portal mount=document().get_element_by_id("app").expect("acces to app") clone:children>
                    <div style=move || if context.open.get() {
                        style.get()
                    } else {"".to_string()}  class=move || format!("{} {}", class.get_value(),
                        match context.open.get() {
                            true => "",
                            false => "hidden"
                        }
                    ) node_ref=content_ref>
                        {children.clone()}
                    </div>
                </Portal>
            </Show>
        </Provider>
    }
}
