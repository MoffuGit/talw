use leptos::prelude::*;

use leptos::ev::Event;
use leptos::html::Span;

use super::ProfilesSettingsContext;

#[component]
pub fn UserAbout() -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let about_preview = RwSignal::new(context.banner.about);
    let span_ref = NodeRef::<Span>::new();
    let textarea_height = RwSignal::new(0);
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
    Effect::new(move |_| {
        if let Some(span) = span_ref.get() {
            textarea_height.set(span.offset_height());
        }
    });
    let focus_textarea = RwSignal::new(false);
    view! {
        <label class="form-control relative px-6 group h-fit w-full grow">
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
                // WARNING: check this
                // <textarea
                // on:input=handle_input
                // on:focus=move |_| focus_textarea.set(true)
                // on:focusout=move |_| focus_textarea.set(false)
                // value=move || about_preview.get().unwrap_or_default()
                // name="new_about"
                // type="text"
                // class=move || {
                // format!(
                // "relative text-lg px-1 w-full resize-none z-50 bg-base-300/60 text-base-content rounded-md {}",
                // { if focus_textarea.get() { "opacity-100" } else { "opacity-0" } },
                // )
                // }
                // style=move || { format!("height: {}px", textarea_height.get()) }
                // />
                <span
                    node_ref=span_ref
                    class=move || {
                        format!(
                            "absolute left-0 top-0 block text-lg rounded-md px-1 w-full whitespace-pre z-40 {} {}",
                            if about_preview.get().is_none() {
                                "bg-base-content/10"
                            } else {
                                "group-hover:bg-base-content/10 text-base-content"
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
