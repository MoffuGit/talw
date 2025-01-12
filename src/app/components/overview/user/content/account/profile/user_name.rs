use crate::app::components::overview::user::content::account::profile::ProfilesSettingsContext;
use leptos::*;

use self::ev::Event;
use self::html::{Input, Span};

#[component]
pub fn UserName() -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let name_preview = create_rw_signal(context.profile.name);

    let input_ref = create_node_ref::<Input>();
    let span_ref = create_node_ref::<Span>();

    let input_width = create_rw_signal(0);

    let handle_input = move |evt: Event| {
        context.user_data_change.set(true);
        name_preview.set(event_target_value(&evt));
        if let Some(span) = span_ref.get() {
            input_width.set(span.offset_width());
        }
    };

    create_effect(move |_| {
        if let Some(span) = span_ref.get() {
            input_width.set(span.offset_width());
        }
    });

    let focus = create_rw_signal(false);

    view! {
        <label class="form-control group relative ml-7 mt-[200px]">
            <span
                class="invisible absolute text-3xl px-2 rounded-md font-bold whitespace-pre"
                node_ref=span_ref
            >
                {move || name_preview.get()}
            </span>
            <div class=move || {
                format!(
                    "label pb-0.5 opacity-0 transition group-hover:opacity-100 {}",
                    { if focus.get() { "opacity-100".to_string() } else { "".to_string() } },
                )
            }>
                <span class="label-text-alt text-sm">"User name"</span>
            </div>
            <input
                value=move || name_preview.get()
                on:input=handle_input
                on:focus=move |_| { focus.set(true) }
                on:focusout=move |_| { focus.set(false) }
                on:keydown=move |evt| {
                    if &evt.key() == "Enter" {
                        evt.prevent_default();
                    }
                }
                name="new_name"
                type="text"
                class="text-3xl font-bold w-8 bg-transparent rounded-md px-2 group-hover:bg-base-300/60 focus:bg-base-300/60 cursor-text transition"
                style=move || format!("width: {}px", input_width.get())
                node_ref=input_ref
            />
        </label>
    }
}
