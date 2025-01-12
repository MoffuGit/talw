use leptos::*;
use leptos_use::use_mouse;
use leptos_use::UseMouseReturn;

use crate::app::components::ui::menu::{MenuContent, MenuProvider, MenuTrigger, TriggerKey};

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[component]
pub fn ContextMenuTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! { <MenuTrigger class=class>{children.map(|children| children())}</MenuTrigger> }
}

#[allow(non_snake_case)]
#[component]
pub fn ContextMenuContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
) -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    let position = Signal::derive(move || {
        format!(
            "translate: {}px {}px;",
            x.get_untracked(),
            y.get_untracked()
        )
    });

    view! {
        <MenuContent
            class=format!("absolute left-0 top-0 pointer-events-auto {}", class)
            ignore=ignore
            style=position
        >
            {children.clone()}
        </MenuContent>
    }
}
