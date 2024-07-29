use crate::app::api::thread::{initial_width, toggle_thread_width};
use leptos::html::Div;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::use_draggable_with_options;
use leptos_use::{use_window, UseDraggableCallbackArgs, UseDraggableOptions, UseDraggableReturn};

#[component]
#[allow(non_snake_case)]
pub fn ThreadSidebar() -> impl IntoView {
    let initial_width = initial_width();
    let update_width = create_action(|width: &f64| toggle_thread_width(*width));

    let divider_ref = create_node_ref::<Div>();
    let UseDraggableReturn { x, .. } = use_draggable_with_options(
        divider_ref,
        UseDraggableOptions::default().on_end(
            move |UseDraggableCallbackArgs {
                      position: Position { x, .. },
                      ..
                  }| {
                let window = use_window();
                if window.is_some() {
                    let window_width = window
                        .as_ref()
                        .unwrap()
                        .inner_width()
                        .expect("should acces to thw window width");
                    if let Some(window_width) = window_width.as_f64() {
                        if x > 0.0 {
                            update_width.dispatch(window_width - x);
                        }
                    }
                }
            },
        ),
    );

    let current_width = Signal::derive(move || {
        let window = use_window();
        if window.is_some() {
            let window_width = window
                .as_ref()
                .unwrap()
                .inner_width()
                .expect("should acces to thw window width");
            if let Some(window_width) = window_width.as_f64() {
                if x.get() > 0.0 {
                    return window_width - x.get();
                }
            }
        }
        initial_width
    });

    view! {
        <div class="w-2 bg-base-300 h-full shrink-0" node_ref=divider_ref/>
        <div class="min-w-[400px] shrink-0" style =move || format!("width: {}px", current_width.get())/>
    }
}
