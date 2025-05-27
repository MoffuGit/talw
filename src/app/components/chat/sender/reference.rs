use leptos::prelude::*;

use crate::app::components::chat::ChatContext;
use crate::app::components::ui::icons::{Icon, IconData};

#[component]
pub fn Reference() -> impl IntoView {
    let ChatContext { msg_reference } =
        use_context::<ChatContext>().expect("should acces to the chat context");
    view! {
        {
            move || {
                msg_reference.get().map(|reference| view!{
                    <div class="relative w-full h-9 bg-base-300 rounded-t-lg border border-b-0 border-base-100 flex items-center p-2 text-sm">
                        <span class="whitespace-pre">
                            "Replying to "
                        </span>
                        <span class="font-semibold">
                            {
                                reference.sender.name
                            }
                        </span>
                        <div on:click=move |_| {
                                msg_reference.set(None);
                            }
                            class="absolute cursor-pointer right-1 w-7 h-7 rounded-md hover:bg-base-100 flex justify-center items-center"
                        >
                            <Icon icon=IconData::X class="w-4 h-4"/>
                        </div>
                    </div>
                })
            }
        }
    }
}
