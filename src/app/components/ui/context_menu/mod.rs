use leptos::ev::contextmenu;
use leptos::html;
use leptos::prelude::*;
use leptos_use::use_document;
use leptos_use::use_event_listener_with_options;
use leptos_use::use_mouse;
use leptos_use::UseEventListenerOptions;
use leptos_use::UseMouseReturn;

use crate::app::components::ui::menu::MenuProviderContext;
use crate::app::components::ui::menu::{MenuContent, MenuProvider, MenuTrigger, TriggerKey};

#[component]
pub fn ContextMenuProvider(
    children: Children,
    #[prop(optional)] open: RwSignal<bool>,
    #[prop(optional)] hidden: RwSignal<bool>,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    view! {
        <MenuProvider
            hidden=hidden
            open=open
            modal=modal
            trigger_ref=trigger_ref
            content_ref=content_ref
            trigger_key=TriggerKey::Rtl
        >
            {children()}
        </MenuProvider>
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! { <MenuTrigger class=class>{children.map(|children| children())}</MenuTrigger> }
}

#[component]
pub fn ContextMenuContent(
    #[prop(optional)] class: &'static str,
    children: ChildrenFn,
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
    #[prop(default = Signal::derive(move || 0.0))] limit_y: Signal<f64>,
) -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    let position = Signal::derive(move || {
        format!(
            "translate: {}px {}px;",
            x.get_untracked(),
            if limit_y.get() < y.get_untracked() {
                limit_y.get()
            } else {
                y.get_untracked()
            }
        )
    });

    let context = use_context::<MenuProviderContext>().expect("acces to menu context");

    view! {
        <MenuContent
            class=format!("absolute left-0 top-0 pointer-events-auto {}", class)
            ignore=ignore
            style=position
        >
            {
                #[cfg(feature = "hydrate")]
                {
                    let _ = use_event_listener_with_options(
                        use_document(),
                        contextmenu,
                        move |evt| {
                            evt.prevent_default();
                            if context.open.get() {
                                context.open.set(false)
                            }
                        },
                        UseEventListenerOptions::default().capture(true),
                    );
                }
                children()
            }
        </MenuContent>
    }
}
