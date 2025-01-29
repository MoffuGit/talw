use leptos::*;
use leptos_router::ActionForm;

use crate::app::api::server::use_server;
use crate::app::components::overview::server::ServerSettingsData;

use self::ev::Event;
use self::html::{Input, Span};

#[component]
pub fn ServerName() -> impl IntoView {
    let edit_server_name = use_server().edit_server_name;
    let server = use_context::<ServerSettingsData>()
        .expect("should acces to the user overview context")
        .server;
    let name_preview = create_rw_signal(server.name);
    let input_ref = create_node_ref::<Input>();
    let span_ref = create_node_ref::<Span>();
    let input_width = create_rw_signal(0);
    let handle_input = move |evt: Event| {
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
    view! {
        <ActionForm action=edit_server_name class="relative ml-44 flex items-center">
            <label class="relative form-control">
                <span
                    class="invisible text-xl absolute px-2 rounded-md whitespace-pre"
                    node_ref=span_ref
                >
                    {move || name_preview.get()}
                </span>
                <div class="label p-0 pb-0.5 transition">
                    <span class="label-text-alt text-sm px-0 pt-0 font-semibold">"Server Name"</span>
                </div>
                <input type="hidden" value=server.id.to_string() name="server_id"/>
                <input
                    value=move || name_preview.get()
                    on:input=handle_input
                    on:keydown=move |evt| {
                        if &evt.key() == "Enter" {
                            evt.prevent_default();
                        }
                    }
                    name="new_name"
                    type="text"
                    class="text-xl w-8 rounded-md px-2 bg-base-content/10 cursor-text transition"
                    style=move || format!("width: {}px", input_width.get())
                    node_ref=input_ref
                />
            </label>
            <button type="submit" class="bg-base-primary rounded-md">
                "Save"
            </button>
        </ActionForm>
    }
}
