use crate::app::components::ui::modal::*;
use leptos::{html, prelude::*};
use uuid::Uuid;

#[component]
pub fn InvitePeopleModal(
    class: &'static str,
    on_click: Signal<()>,
    invite_code: Uuid,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let invite_ref = NodeRef::<html::Div>::new();
    view! {
        <ModalProvider content_ref=content_ref>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <div class="w-full p-4 flex-col shadow-sm shadow-base-300/80">
                    <div class="font-bold text-[16px] leading-[30px]">"Invite friends to you"</div>
                    <div class="mt-2 p-1 rounded bg-base-300/50 w-full h-8"></div>
                </div>
                <div class="w-full p-4 flex-col bg-base-200">
                    <div class="font-bold text-[12px] mb-1 leading-[30px] uppercase">
                        "or, send a server invite link to a friend"
                    </div>
                    <div class="text-base w-full rounded h-[40px] bg-base-300 flex items-center justify-between">
                        <div class="p-2" node_ref=invite_ref>
                            {invite_code.simple().to_string()}
                        </div>
                        <div class="btn-primary text-primary-content w-[75px] h-[32px] rounded m-1">
                            // on:click=move |_| {
                            // invite_ref.get().map(|node| {
                            // let text = node.inner_text();
                            // let window = use_window();
                            // let navigator = window.navigator();
                            // navigator.map(|navigator| navigator.clipboard().map(|clip| clip.write_text(text)));
                            // });
                            // }
                            "Copy"
                        </div>
                    </div>
                </div>
            </ModalContent>
        </ModalProvider>
    }
}
