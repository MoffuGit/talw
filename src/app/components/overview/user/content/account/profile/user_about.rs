use leptos::*;

use self::ev::Event;
use self::html::Span;

use super::ProfilesSettingsContext;

#[component]
pub fn UserAbout() -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let about_preview = create_rw_signal(context.banner.about);
    let span_ref = create_node_ref::<Span>();
    let textarea_height = create_rw_signal(0);
    let handle_input = move |evt: Event| {
        context.user_data_change.set(true);
        let value = event_target_value(&evt);
        if value.is_empty() {
            about_preview.set(None);
        } else {
            about_preview.set(Some(value))
        }
        if let Some(span) = span_ref.get() {
            textarea_height.set(span.offset_height());
        }
    };
    create_effect(move |_| {
        if let Some(span) = span_ref.get() {
            textarea_height.set(span.offset_height());
        }
    });
    let focus_textarea = create_rw_signal(false);
    view! {
        <label class="form-control relative px-8 group h-fit w-full grow">
            <div class=move || {
                format!(
                    "label py-0.5 transition transition {}",
                    {
                        if about_preview.get().is_some() {
                            "opacity-0 group-hover:opacity-100"
                        } else {
                            ""
                        }
                    },
                )
            }>
                <span class="label-text">"About me"</span>
            </div>
            <div class="relative h-fit w-full">
                <textarea
                    value=move || about_preview.get().unwrap_or_default()
                    on:input=handle_input
                    on:focus=move |_| focus_textarea.set(true)
                    on:focusout=move |_| focus_textarea.set(false)
                    name="new_about"
                    type="text"
                    class=move || {
                        format!(
                            "relative text-lg px-1 w-full resize-none z-50 bg-base-300/60 text-base-content rounded-md {}",
                            { if focus_textarea.get() { "opacity-100" } else { "opacity-0" } },
                        )
                    }
                    style=move || { format!("height: {}px", textarea_height.get()) }
                />
                <span
                    node_ref=span_ref
                    class=move || {
                        format!(
                            "absolute left-0 top-0 block text-lg rounded-md px-1 w-full whitespace-pre z-40 {} {}",
                            if about_preview.get().is_none() {
                                "bg-base-300/60 text-base-content/80"
                            } else {
                                "group-hover:bg-base-300/60 text-base-content"
                            },
                            if focus_textarea.get() { "invisible" } else { "" },
                        )
                    }
                >
                    {move || {
                        let mut about = about_preview.get().unwrap_or_default();
                        about.push(' ');
                        about
                    }}
                </span>
            </div>
        </label>
    }
}
