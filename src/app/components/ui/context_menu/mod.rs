use leptos::*;
use leptos_use::use_mouse;
use leptos_use::UseMouseReturn;

use crate::app::components::ui::menu::{MenuContent, MenuTrigger, ProvideMenu, TriggerKey};

#[allow(non_snake_case)]
#[component]
pub fn ContextMenuProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional, default = true)] modal: bool,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));
    let trigger_ref = create_node_ref::<html::Div>();
    let content_ref = create_node_ref::<html::Div>();
    view! {
        <ProvideMenu open=open modal=modal trigger_ref=trigger_ref content_ref=content_ref trigger_key=TriggerKey::Rtl>
            {children()}
        </ProvideMenu>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ContextMenuTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <MenuTrigger class=class>
            {children.map(|children| children())}
        </MenuTrigger>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ContextMenuContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
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
        <MenuContent class=format!("absolute left-0 top-0 pointer-events-auto {}", class) style=position>
            {children.clone()}
        </MenuContent>
    }
}
