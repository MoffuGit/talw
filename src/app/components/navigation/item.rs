use leptos::*;
use leptos_router::{use_router, A};
use uuid::Uuid;

#[component]
pub fn Item(id: Uuid, name: String) -> impl IntoView {
    let current_server = move || {
        use_router().pathname().with(|path| {
            Uuid::parse_str(path.split('/').nth(2).unwrap_or_default()).unwrap_or_default()
        })
    };
    view! {
        <div class="tooltip tooltip-right" /*font-medium before:bg-white*/ data-tip=name>
            <A href=id.simple().to_string() class="group relative flex items-center">
                <div class=move || format!("absolute left-0 bg-primary rounded-r-full transition-all w-[4px] {}", {
                    match current_server() == id {
                        false => "group-hover:h-[20px] h-[8px]",
                        true =>"h-[36px]",
                    }
                })
                />
                <div class="relative mx-3 h-[48px] transition-all bg-neutral-focus text-neutral-content rounded-[24px] group-hover:rounded-[16px] w-[48px] overflow-hidden"/>
            </A>
        </div>
    }
}
