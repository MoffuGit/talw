use leptos::*;
use leptos_use::{use_document, use_event_listener};

use self::ev::{keydown, KeyboardEvent};

#[component]
pub fn OverviewTrigger(
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] on_click: Option<Signal<()>>,
    #[prop(optional)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div
            on:click=move |_| {
                open.set(true);
                if let Some(on_click) = on_click {
                    on_click.get();
                }
            }
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn OverviewClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] on_click: Option<Signal<()>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            {..attrs}
            on:click=move |_| {
                if let Some(on_click) = on_click {
                    on_click.get()
                }
                open.set(false);
            }
            class=class
        >
            {children.map(|children| children())}
        </button>
    }
}

#[component]
pub fn OverviewContent(
    children: ChildrenFn,
    class: &'static str,
    open: RwSignal<bool>,
    #[prop(optional)] on_close: Option<Signal<()>>,
) -> impl IntoView {
    create_effect(move |_| {
        if !open.get() {
            if let Some(on_close) = &on_close {
                on_close.get();
            }
        }
    });
    view! {
        <Show when=move || open.get()>
            <Portal
                mount=document().get_element_by_id("app").expect("acces to the app")
                clone:children
            >
                {
                    let _ = use_event_listener(
                        use_document(),
                        keydown,
                        move |evt: KeyboardEvent| {
                            if evt.key() == "Escape" {
                                open.set(false)
                            }
                        },
                    );
                }
                <div class=format!("z-[999] absolute inset-0 {}", class)>{children.clone()}</div>
            </Portal>
        </Show>
    }
}
