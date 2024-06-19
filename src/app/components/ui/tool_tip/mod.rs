use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use std::time::Duration;
use web_sys::PointerEvent;

#[derive(Clone)]
pub struct TooltipProviderContext {
    // content_id: String,
    is_open: RwSignal<bool>,
    on_trigger_leave: Signal<()>,
    on_trigger_enter: Signal<()>,
    on_open: Signal<()>,
    on_close: Signal<()>,
    trigger_ref: NodeRef<html::Div>,
}

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(default = Duration::new(3,0))] delay_duration: Duration,
) -> impl IntoView {
    let was_open_delayed_ref = create_rw_signal(false);
    let is_open = create_rw_signal(false);
    let open_timer_ref: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);
    let trigger_ref = create_node_ref::<html::Div>();

    let handle_open = move || match open_timer_ref.get_untracked() {
        None => {
            was_open_delayed_ref.update_untracked(|value| *value = false);
            is_open.update(|value| *value = true);
        }
        Some(timer) => {
            timer.clear();
            was_open_delayed_ref.update_untracked(|value| *value = false);
            is_open.update(|value| *value = true);
        }
    };

    let handle_close = move || match open_timer_ref.get_untracked() {
        None => {
            is_open.update(|value| *value = false);
        }
        Some(timer) => {
            timer.clear();
            is_open.update(|value| *value = false);
        }
    };

    let handle_delayed_open = move || match open_timer_ref.get_untracked() {
        None => {
            open_timer_ref.update_untracked(|value| {
                *value = set_timeout_with_handle(
                    move || {
                        was_open_delayed_ref.update_untracked(|value| *value = true);
                        is_open.update(|value| *value = true);
                    },
                    delay_duration,
                )
                .ok()
            });
        }
        Some(timer) => {
            timer.clear();
            open_timer_ref.update_untracked(|value| {
                *value = set_timeout_with_handle(
                    move || {
                        was_open_delayed_ref.update_untracked(|value| *value = true);
                        is_open.update(|value| *value = true);
                    },
                    delay_duration,
                )
                .ok()
            });
        }
    };

    let on_trigger_enter = Signal::derive(handle_delayed_open);
    let on_trigger_leave = Signal::derive(move || match open_timer_ref.get_untracked() {
        None => {
            handle_close();
        }
        Some(timer) => {
            handle_close();
            timer.clear();
        }
    });
    let on_open = Signal::derive(handle_open);
    let on_close = Signal::derive(handle_close);
    provide_context(TooltipProviderContext {
        is_open,
        on_trigger_leave,
        on_trigger_enter,
        on_open,
        on_close,
        trigger_ref,
    });

    children()
}

#[component]
pub fn TooltipTrigger(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let provider_context = use_context::<TooltipProviderContext>().expect("have this context");
    let is_hover = create_rw_signal(false);
    let trigger_ref = provider_context.trigger_ref;

    view! {
        <div class=class
            _ref=trigger_ref
            on:pointermove=move |evt: PointerEvent| {
                if evt.pointer_type() == "touch" {
                    return;
                }
                if !is_hover.get_untracked() {
                    provider_context.on_trigger_enter.get_untracked();
                    is_hover.update_untracked(|value| *value = true)
                }
            }
            on:pointerleave=move |_| {
                provider_context.on_trigger_leave.get_untracked();
                is_hover.update_untracked(|value| *value = false)
            }
            on:click=move |evt| {
                evt.stop_propagation();
                provider_context.on_close.get_untracked();
            }
            on:wheel=move |_| {
                provider_context.on_close.get_untracked();
            }
            on:focus=move |_| {
                provider_context.on_open.get_untracked();
            }
        >
        {children()}
        </div>
    }
}

#[derive(Clone, Copy)]
pub enum ToolTipSide {
    Bottom,
    Left,
    Right,
    Top,
}

pub fn get_tooltip_position(
    trigger: HtmlElement<html::Div>,
    content: HtmlElement<html::Div>,
    tooltip_side: ToolTipSide,
    tooltip_of_side: f64,
) -> (String, String) {
    let trigger_values = trigger.get_bounding_client_rect();
    let content_height = content.offset_height();
    let content_width = content.offset_width();
    let (y, height) = (trigger_values.y(), trigger.offset_height());
    let (x, width) = (trigger_values.x(), trigger.offset_width());
    match tooltip_side {
        ToolTipSide::Bottom => {
            let y = y + f64::from(width) + tooltip_of_side;
            let x = x + (f64::from(width) / 2.0) - (f64::from(content_width) / 2.0);
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Left => {
            let y = y + (f64::from(height) / 2.0) - (f64::from(content_height) / 2.0);
            let x = x - f64::from(width) - tooltip_of_side;
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Right => {
            let y = y + (f64::from(height) / 2.0) - (f64::from(content_height) / 2.0);
            let x = x + f64::from(width) + tooltip_of_side;
            (x.to_string(), y.to_string())
        }
        ToolTipSide::Top => {
            let y = y - f64::from(width) - tooltip_of_side;
            let x = x + (f64::from(width) / 2.0) - (f64::from(content_width) / 2.0);
            (x.to_string(), y.to_string())
        }
    }
}

#[component]
pub fn TooltipContent(
    tip: String,
    #[prop(optional)] class: &'static str,
    #[prop(optional, default = ToolTipSide::Right)] tooltip_side: ToolTipSide,
    #[prop(optional, default = 2.0)] tooltip_of_side: f64,
) -> impl IntoView {
    let context = use_context::<TooltipProviderContext>().expect("is open context");

    let is_open = context.is_open;
    let trigger_ref = context.trigger_ref;

    let content_ref = create_node_ref::<html::Div>();

    let visibility = move || {
        if !is_open.get() {
            "visibility: hidden"
        } else {
            ""
        }
    };

    let show = create_rw_signal(false);

    let position = move || {
        if let (Some(trigger), Some(content), true) =
            (trigger_ref.get(), content_ref.get(), is_open.get())
        {
            get_tooltip_position(trigger, content, tooltip_side, tooltip_of_side)
        } else {
            ("".to_string(), "".to_string())
        }
    };

    create_effect(move |_| show.update(|value| *value = true));

    view! {
        <Show when=move || show.get()>
            <Portal mount=document().get_element_by_id("app").unwrap() clone:tip>
                <div _ref=content_ref style=move || format!("translate: {}px {}px; {}", position().0, position().1, visibility()) class=format!("absolute z-50 left-0 top-0 animate-tooltip-open {}", class)>
                    {tip.clone()}
                </div>
            </Portal>
        </Show>
    }
}
