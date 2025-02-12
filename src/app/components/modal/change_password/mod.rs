use crate::app::components::ui::modal::*;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn ChangePasswordModal(
    _user_id: Uuid,
    #[prop(optional)] children: Option<Children>,
    class: &'static str,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let content_ref = NodeRef::new();
    let _new_password = RwSignal::new(String::new());
    view! {
        <ModalProvider open=open content_ref=content_ref>
            <ModalTrigger class=class>{children.map(|children| children())}</ModalTrigger>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div>"Update your password"</div>
                <div>"Enter you current password and the new password"</div>
                <div>"old"</div>
                <div>"new"</div>
                <div>"confirm new"</div>
            </ModalContent>
        </ModalProvider>
    }
}
