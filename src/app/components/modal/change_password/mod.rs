use crate::app::components::ui::modal::*;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn ChangePasswordModal(
    _user_id: Uuid,
    #[prop(optional)] children: Option<Children>,
    class: &'static str,
) -> impl IntoView {
    let open = create_rw_signal(false);
    let content_ref = create_node_ref();
    let _new_password = create_rw_signal(String::new());
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
