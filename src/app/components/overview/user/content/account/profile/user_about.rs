use std::time::Duration;

use leptos::*;
use leptos_router::ActionForm;

use crate::app::api::user::use_user;

use self::ev::Event;
use self::html::Form;
use self::leptos_dom::helpers::TimeoutHandle;

use super::ProfilesSettingsContext;

#[component]
pub fn UserAbout() -> impl IntoView {
    let edit_user_about = use_user().edit_banner_about;
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let about_preview = create_rw_signal(context.banner.about);
    let timer_ref: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);
    let form_ref = create_node_ref::<Form>();
    let handle_input = move |evt: Event| {
        about_preview.set(Some(event_target_value(&evt)));
        if let Some(timer) = timer_ref.get() {
            timer.clear();
        }
        timer_ref.set(
            set_timeout_with_handle(
                move || {
                    form_ref.get().map(|form| form.request_submit());
                },
                Duration::new(3, 0),
            )
            .ok(),
        );
    };
    view! {
        <ActionForm action=edit_user_about>
            <textarea value=move || about_preview.get() on:change=handle_input name="new_about" type="text" rows=5 cols=30 class="mx-2 textarea mt-2 text-xl font-bold bg-transparent py-2" />
        </ActionForm>
    }
}
