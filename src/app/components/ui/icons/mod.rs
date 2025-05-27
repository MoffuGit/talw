use leptos::prelude::*;

#[derive(Debug, Clone)]
pub enum IconData {
    X,
    Sticker,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronTop,
    CirclePlus,
    Plus,
    Search,
    Command,
    MessageCircle,
    Inbox,
    PanelLeft,
    Pin,
    Users,
    ListTree,
}

impl IconData {
    fn view(&self) -> impl IntoView {
        match self {
            IconData::X => view!{
                <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
            }.into_any(),
            IconData::Command => view!{
                <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"/>
            }.into_any(),
            IconData::Plus => view!{
                <path d="M5 12h14"/><path d="M12 5v14"/>
            }.into_any(),
            IconData::ChevronRight => view!{<path d="m9 18 6-6-6-6"/>}.into_any(),
            IconData::ChevronTop => view!{<path d="m18 15-6-6-6 6"/>}.into_any(),
            IconData::ChevronLeft => view!{
                <path d="m15 18-6-6 6-6"/>
            }.into_any(),
            IconData::ChevronDown => view!{
                <path d="m6 9 6 6 6-6"/>
            }.into_any(),
            IconData::Sticker => view! {
                <path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"/><path d="M14 3v4a2 2 0 0 0 2 2h4"/><path d="M8 13h.01"/><path d="M16 13h.01"/><path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"/>
            }
            .into_any(),
            IconData::CirclePlus => view! {
                <circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>
            }
            .into_any(),
            IconData::Search => view!{
                <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
            }.into_any(),
            IconData::MessageCircle => view!{
                <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>
            }.into_any(),
            IconData::Inbox => view! {
                <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>
            }.into_any(),
            IconData::PanelLeft => view!{
                <rect width="18" height="18" x="3" y="3" rx="2" />
                <path d="M9 3v18" />
            }.into_any(),
            IconData::Pin => view!{
                <path d="M12 17v5"/><path d="M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z"/>
            }.into_any(),
            IconData::Users => view!{
                <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>
            }.into_any(),
            IconData::ListTree => view!{
                <path d="M21 12h-8" />
                <path d="M21 6H8" />
                <path d="M21 18h-8" />
                <path d="M3 6v4c0 1.1.9 2 2 2h3" />
                <path d="M3 10v6c0 1.1.9 2 2 2h3" />
            }.into_any()
        }
    }
}

#[component]
pub fn Icon(
    #[prop(into)] icon: Signal<IconData>,
    #[prop(into, optional)] class: Signal<String>,
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
