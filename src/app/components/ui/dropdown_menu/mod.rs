use leptos::*;
use leptos_use::{use_mouse_in_element, UseMouseInElementReturn};

use crate::app::components::ui::menu::{MenuContent, MenuTrigger, ProvideMenu};

#[derive(Clone)]
pub struct DropdownProviderContext {
    open: RwSignal<bool>,
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
}
#[component]
pub fn DropdownProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional, default = true)] modal: bool,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));
    let trigger_ref = create_node_ref::<html::Div>();
    let content_ref = create_node_ref::<html::Div>();
    provide_context(DropdownProviderContext {
        open,
        trigger_ref,
        content_ref,
    });
    view! {
        <ProvideMenu open=open modal=modal trigger_ref=trigger_ref content_ref=content_ref>
            {children()}
        </ProvideMenu>
    }
}

#[component]
pub fn DropdownTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <MenuTrigger class=class>
            {children.map(|children| children())}
        </MenuTrigger>
    }
}

#[derive(Clone)]
enum MenuSide {
    Bottom,
    Left,
    Right,
    Top,
}

#[derive(Clone)]
enum MenuAlign {
    Start,
    Center,
    End,
}

#[component]
pub fn DropdownContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    // #[prop(optional, default = MenuSide::Bottom)] side: MenuSide,
    // #[prop(optional, default = 0)] side_of_set: usize,
    // #[prop(optional, default = MenuAlign::Center)] align: MenuAlign,
    // #[prop(optional, default = 0)] align_of_set: usize,
) -> impl IntoView {
    let context =
        use_context::<DropdownProviderContext>().expect("acces to DropdownProviderContext");
    let trigger_ref = context.trigger_ref;
    let content_ref = context.content_ref;
    let UseMouseInElementReturn {
        element_width: content_width,
        // element_height: content_heigt,
        ..
    } = use_mouse_in_element(content_ref);
    let UseMouseInElementReturn {
        element_width: trigger_width,
        element_height: trigger_height,
        element_position_x: trigger_position_x,
        element_position_y: trigger_position_y,
        ..
    } = use_mouse_in_element(trigger_ref);
    let content_postion_x = move || {
        trigger_position_x.get() + (trigger_width.get() / 2.0) - (content_width.get() / 2.0)
    };
    let content_postion_y = move || trigger_position_y.get() + trigger_height.get() /* + content_heigt.get() */;

    let position = Signal::derive(move || {
        format!(
            "translate: {}px {}px;",
            content_postion_x(),
            content_postion_y()
        )
    });
    // let class = format!("absolute left-0 top-0 {}", class);

    view! {
        <MenuContent class=format!("absolute left-0 top-0 pointer-events-auto {}", class) style=position>
            {children.clone()}
        </MenuContent>
    }
}
