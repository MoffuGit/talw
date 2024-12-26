use leptos::*;
use leptos_icons::Icon;
use leptos_router::ActionForm;

use crate::app::api::user::use_user;
use crate::app::components::overview::user::content::account::profile::ProfilesSettingsContext;

use self::ev::Event;
use self::html::{Form, Input, Span};

#[component]
pub fn UserName() -> impl IntoView {
    let edit_user_name = use_user().edit_profile_name;
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let name_preview = create_rw_signal(context.profile.name);

    let form_ref = create_node_ref::<Form>();
    let input_ref = create_node_ref::<Input>();
    let span_ref = create_node_ref::<Span>();

    let input_width = create_rw_signal(0);

    let disable_input = create_rw_signal(true);

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

    let click_edit = move |_| {
        disable_input.set(false);
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
            let lenght = name_preview.get().len();
            let _ = input.set_selection_range(lenght as u32, lenght as u32);
        }
    };

    view! {
        <ActionForm action=edit_user_name node_ref=form_ref class="relative ml-7 mt-52 flex items-center group w-fit">
            <span class="invisible absolute text-3xl px-2 font-bold whitespace-pre"  node_ref=span_ref>
                {move || name_preview.get()}
            </span>
            <input
                value=move || name_preview.get()
                on:input=handle_input
                on:focusout = move |_| disable_input.set(true)
                disabled=move || disable_input.get()
                name="new_name"
                type="text"
                class="text-3xl px-2 font-bold w-8 mr-2 bg-transparent"
                style=move || format!("width: {}px", input_width.get())
                node_ref=input_ref
            />
                {
                    move || {
                        if disable_input.get() {
                            view!{
                                <div class="w-7 h-7">
                                    <div on:click=click_edit class="w-7 h-7 hidden group-hover:flex rounded-full bg-base-content/20 flex items-center justify-center">
                                        <Icon icon=icondata::RiEditDesignFill class="w-5 h-5 fill-base-content/50"/>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view!{
                                <div class="w-7 h-7 mr-2 flex rounded-full bg-base-content/20 flex items-center justify-center">
                                    <Icon icon=icondata::RiRefreshSystemFill on:pointerdown=move |evt| evt.prevent_default() class="w-5 h-5 fill-base-content/50"/>
                                </div>
                                <div class="w-7 h-7 flex rounded-full bg-base-content/20 flex items-center justify-center">
                                    <Icon icon=icondata::RiCheckSystemFill on:pointerdown=move |evt| evt.prevent_default() class="w-5 h-5 fill-base-content/50"/>
                                </div>
                            }.into_view()
                        }
                    }
                }
        </ActionForm>
    }
}
