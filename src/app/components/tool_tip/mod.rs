use leptos::{html::Button, leptos_dom::helpers::TimeoutHandle, *};
use std::time::Duration;
use web_sys::PointerEvent;

#[component]
pub fn TooltipPortal(children: ChildrenFn) -> impl IntoView {
    let (show, set_show) = create_signal(false);
    create_effect(move |_| set_show(true));

    let children = store_value(children);
    view! {
        <Show when=move || show.get()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
                {children.with_value(|children| children())}
            </Portal>
        </Show>
    }
}

#[derive(PartialEq)]
pub enum TooltipState {
    Open,
    DelayedOpen,
    Closed,
}

#[derive(Clone)]
pub struct TooltipProviderContext {
    // content_id: String,
    is_open: RwSignal<bool>,
    state_attribute: Memo<TooltipState>,
    on_trigger_leave: Signal<()>,
    on_trigger_enter: Signal<()>,
    on_open: Signal<()>,
    on_close: Signal<()>,
}

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(default = Duration::new(3,0))] delay_duration: Duration,
) -> impl IntoView {
    let was_open_delayed_ref = create_rw_signal(false);
    let is_open = create_rw_signal(false);
    let open_timer_ref: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);

    let state_attribute =
        create_memo(
            move |_| match (is_open.get(), was_open_delayed_ref.get_untracked()) {
                (true, false) => TooltipState::Open,
                (true, true) => TooltipState::DelayedOpen,
                _ => TooltipState::Closed,
            },
        );

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
        state_attribute,
        on_trigger_leave,
        on_trigger_enter,
        on_open,
        on_close,
    });

    create_effect(move |_| {
        log::info!("{}", is_open.get());
    });
    view! {
        {children()}
    }
}

#[component]
pub fn TooltipTrigger(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let provider_context = use_context::<TooltipProviderContext>().expect("have this context");
    let is_hover = create_rw_signal(false);

    view! {
        <div class=class
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
            on:click=move |_| {
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
