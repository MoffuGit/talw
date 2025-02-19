use crate::app::api::category::use_category;
use crate::app::components::ui::modal::*;
use crate::app::ActionForm;
use crate::entities::category::Category;
use leptos::{html, prelude::*};
use uuid::Uuid;

 
#[component]
pub fn DeleteCategoryModal(
    category: Category,
    class: &'static str,
    server_id: Uuid,
    on_click: Signal<()>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let delete_category = use_category().delete_category;
    view! {
        <ModalProvider content_ref=content_ref>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <h2 class="p-4  leading-[24px] text-[20px] font-bold text-start w-full">
                    {format!("Delete '{}'", category.name)}
                </h2>
                <div class="px-4 pb-10 w-full">
                    {format!(
                        "Are you sure you want to delete {}? This cannnot be undone.",
                        category.name,
                    )}
                </div>
                <div class="relative p-4 flex justify-end w-full bg-base-300/80">
                    <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                        "Cancel"
                    </ModalClose>
                    <ActionForm action=delete_category>
                        <input value=category.id.to_string() type="hidden" name="category_id" />
                        <input value=server_id.to_string() type="hidden" name="server_id" />
                        <button
                            type="submit"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-error text-error-content"
                            disabled=move || delete_category.pending().get()
                        >
                            "Delete Category"
                        </button>
                    </ActionForm>
                </div>
            </ModalContent>
        </ModalProvider>
    }
}
