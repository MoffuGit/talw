use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_use::use_element_bounding;
use leptos_use::UseElementBoundingReturn;

use crate::app::components::ui::menu::{MenuContent, MenuProvider, MenuTrigger, TriggerKey};

#[derive(Clone)]
pub struct DropdownProviderContext {
    trigger_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    open: RwSignal<bool>,
}

#[component]
pub fn DropdownProvider(
    children: Children,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional)] trigger_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] content_ref: Option<NodeRef<html::Div>>,
    #[prop(optional)] hidden: Option<RwSignal<bool>>,
) -> impl IntoView {
    let open = open.unwrap_or(RwSignal::new(false));
    let trigger_ref = trigger_ref.unwrap_or_default();
    let content_ref = content_ref.unwrap_or_default();
    let hidden = hidden.unwrap_or(RwSignal::new(false));
    view! {
        <Provider value=DropdownProviderContext {
            trigger_ref,
            content_ref,
            open,
        }>
            <MenuProvider
                hidden=hidden
                open=open
                modal=modal
                trigger_ref=trigger_ref
                content_ref=content_ref
                trigger_key=TriggerKey::Ltr
            >
                {children()}
            </MenuProvider>
        </Provider>
    }
}

#[component]
pub fn DropdownTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! { <MenuTrigger class=class>{children.map(|children| children())}</MenuTrigger> }
}

#[derive(Clone, Copy)]
pub enum MenuSide {
    Bottom,
    Left,
    Right,
    Top,
}

#[derive(Clone)]
pub enum MenuAlign {
    Start,
    Center,
    End,
}

pub struct MenuPositionReturn<U>
where
    U: Fn() + Clone,
{
    x: Signal<f64>,
    y: Signal<f64>,
    update_trigger: U,
}

fn use_menu_position(
    content_ref: NodeRef<html::Div>,
    trigger_ref: NodeRef<html::Div>,
    side: MenuSide,
    side_of_set: f64,
    align: MenuAlign,
    align_of_set: f64,
) -> MenuPositionReturn<impl Fn() + Clone> {
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
        update: update_trigger,
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
                        MenuAlign::End => -(content_width.get()) + align_of_set,
                    }
            };
            let content_position_y = move || trigger_position_y.get() + trigger_height.get() + side_of_set /* + content_heigt.get() */;
            MenuPositionReturn {
                update_trigger,
                x: Signal::derive(content_position_x),
                y: Signal::derive(content_position_y),
            }
        }
        MenuSide::Left => {
            let content_position_x =
                move || trigger_position_x.get() - content_width.get() - side_of_set;
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
            MenuPositionReturn {
                update_trigger,
                x: Signal::derive(content_position_x),
                y: Signal::derive(content_position_y),
            }
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
            MenuPositionReturn {
                update_trigger,
                x: Signal::derive(content_position_x),
                y: Signal::derive(content_position_y),
            }
        }
        MenuSide::Top => {
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
            let content_position_y = move || trigger_position_y.get() - content_heigt.get() + side_of_set /* + content_heigt.get() */;
            MenuPositionReturn {
                update_trigger,
                x: Signal::derive(content_position_x),
                y: Signal::derive(content_position_y),
            }
        }
    }
}

#[component]
pub fn DropdownContent(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, default = MenuSide::Bottom)] side: MenuSide,
    #[prop(optional, default = 0.0)] side_of_set: f64,
    #[prop(optional, default = MenuAlign::Center)] align: MenuAlign,
    #[prop(optional, default = 0.0)] align_of_set: f64,
    #[prop(default = None)] limit_y: Option<f64>,
    #[prop(optional)] ignore: Vec<NodeRef<html::Div>>,
) -> impl IntoView {
    let context =
        use_context::<DropdownProviderContext>().expect("acces to DropdownProviderContext");
    let content_ref = context.content_ref;
    let trigger_ref = context.trigger_ref;
    let MenuPositionReturn {
        x,
        y,
        update_trigger,
    } = use_menu_position(
        content_ref,
        trigger_ref,
        side,
        side_of_set,
        align,
        align_of_set,
    );

    let y_position = move || {
        if limit_y.is_some_and(|limit_y| y.get() > limit_y) {
            limit_y.unwrap()
        } else {
            y.get()
        }
    };

    let position = Signal::derive(move || format!("translate: {}px {}px;", x.get(), y_position()));

    Effect::new(move |_| {
        if context.open.get() {
            update_trigger();
        }
    });

    view! {
        <MenuContent
            class=format!("absolute left-0 top-0 pointer-events-auto {}", class)
            ignore=ignore
            style=position
        >
            {children.clone().map(|children| children())}
        </MenuContent>
    }
}
