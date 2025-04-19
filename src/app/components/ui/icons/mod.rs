use leptos::prelude::*;

#[derive(Debug, Clone)]
pub enum IconData {
    Sticker,
    CirclePlus,
}

impl IconData {
    fn view(&self) -> impl IntoView {
        match self {
            IconData::Sticker => view! {
                <path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"/><path d="M14 3v4a2 2 0 0 0 2 2h4"/><path d="M8 13h.01"/><path d="M16 13h.01"/><path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"/>
            }
            .into_any(),
            IconData::CirclePlus => view! {
                <circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>
            }
            .into_any(),
        }
    }
}

#[component]
pub fn Icon(
    #[prop(into)] icon: Signal<IconData>,
    #[prop(into)] class: Signal<&'static str>,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
            {move || icon.get().view()}
        </svg>
    }
}
