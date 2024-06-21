use leptos::*;
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

use crate::app::components::ui::menu::{MenuContent, MenuTrigger, ProvideMenu, TriggerKey};

#[derive(Clone)]
pub struct DropdownProviderContext {
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
}

#[allow(non_snake_case)]
#[component]
pub fn DropdownProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional, default = true)] modal: bool,
) -> impl IntoView {
    let open = open.unwrap_or(create_rw_signal(false));
    let trigger_ref = create_node_ref::<html::Div>();
    let content_ref = create_node_ref::<html::Div>();
    view! {
        <Provider value=DropdownProviderContext {
        trigger_ref,
        content_ref,
        }>
            <ProvideMenu open=open modal=modal trigger_ref=trigger_ref content_ref=content_ref trigger_key=TriggerKey::Ltr>
                {children()}
            </ProvideMenu>
        </Provider>
    }
}

#[allow(non_snake_case)]
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

#[allow(dead_code)]
#[derive(Clone)]
pub enum MenuSide {
    Bottom,
    Left,
    Right,
    Top,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum MenuAlign {
    Start,
    Center,
    End,
}

fn use_menu_position(
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    side: MenuSide,
    side_of_set: f64,
    align: MenuAlign,
    align_of_set: f64,
) -> Signal<(f64, f64)> {
    let UseElementBoundingReturn {
        width: content_width,
        height: content_heigt,
        ..
    } = use_element_bounding(content_ref);
    let UseElementBoundingReturn {
        width: trigger_width,
        height: trigger_height,
        x: trigger_position_x,
        y: trigger_position_y,
        ..
    } = use_element_bounding(trigger_ref);
    match side {
        MenuSide::Bottom => {
            let content_position_x = move || {
                trigger_position_x.get()
                    + match align {
                        MenuAlign::Start => align_of_set,
                        MenuAlign::Center => {
                            (trigger_width.get() / 2.0) - (content_width.get() / 2.0)
                        }
                        MenuAlign::End => trigger_width.get() + align_of_set,
                    }
            };
            let content_position_y = move || trigger_position_y.get() + trigger_height.get() + side_of_set /* + content_heigt.get() */;
            Signal::derive(move || (content_position_x(), content_position_y()))
        }
        MenuSide::Left => {
            let content_position_x =
                move || trigger_position_x.get() - content_width.get() + side_of_set;
            let content_position_y = move || {
                trigger_position_y.get()
                    + match align {
                        MenuAlign::Start => align_of_set,
                        MenuAlign::Center => {
                            (trigger_height.get() / 2.0) - (content_heigt.get() / 2.0)
                        }
                        MenuAlign::End => trigger_height.get(),
                    }
            };
            Signal::derive(move || (content_position_x(), content_position_y()))
        }
        MenuSide::Right => {
            let content_position_x =
                move || trigger_position_x.get() + trigger_width.get() + side_of_set;
            let content_position_y = move || {
                trigger_position_y.get()
                    + match align {
                        MenuAlign::Start => align_of_set,
                        MenuAlign::Center => {
                            (trigger_height.get() / 2.0) - (content_heigt.get() / 2.0)
                        }
                        MenuAlign::End => trigger_height.get(),
                    }
            };
            Signal::derive(move || (content_position_x(), content_position_y()))
        }
        MenuSide::Top => {
            let content_postion_x = move || {
                trigger_position_x.get()
                    + match align {
                        MenuAlign::Start => align_of_set,
                        MenuAlign::Center => {
                            (trigger_width.get() / 2.0) - (content_width.get() / 2.0)
                        }
                        MenuAlign::End => trigger_width.get() + align_of_set,
                    }
            };
            let content_postion_y = move || trigger_position_y.get() - content_heigt.get() + side_of_set /* + content_heigt.get() */;
            Signal::derive(move || (content_postion_x(), content_postion_y()))
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn DropdownContent(
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, default = MenuSide::Bottom)] side: MenuSide,
    #[prop(optional, default = 0.0)] side_of_set: f64,
    #[prop(optional, default = MenuAlign::Center)] align: MenuAlign,
    #[prop(optional, default = 0.0)] align_of_set: f64,
) -> impl IntoView {
    let context =
        use_context::<DropdownProviderContext>().expect("acces to DropdownProviderContext");
    let trigger_ref = context.trigger_ref;
    let content_ref = context.content_ref;
    let use_menu_position = use_menu_position(
        trigger_ref,
        content_ref,
        side,
        side_of_set,
        align,
        align_of_set,
    );

    let position = Signal::derive(move || {
        format!(
            "translate: {}px {}px;",
            use_menu_position().0,
            use_menu_position().1,
        )
    });

    view! {
        <MenuContent class=format!("absolute left-0 top-0 pointer-events-auto {}", class) style=position>
            {children.clone()}
        </MenuContent>
    }
}
