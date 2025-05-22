use crate::app::components::chat::messages::pin::Pin;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::message::ChannelMessage;
use crate::entities::server::ServerStoreFields;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_document;
use uuid::Uuid;

#[component]
pub fn MessageContextMenu(
    #[prop(into)] member_id: Signal<Uuid>,
    message: RwSignal<ChannelMessage>,
    children: Children,
) -> impl IntoView {
    let current_server = use_current_server_context();
    let current_member = current_server.member;
    let content_ref: NodeRef<Div> = NodeRef::new();
    let limit_y = Signal::derive(move || {
        let content_height = {
            if let Some(node) = content_ref.get() {
                node.offset_height() as f64
            } else {
                Default::default()
            }
        };
        use_document()
            .body()
            .map(|body| (body.get_bounding_client_rect().height() - content_height) - 4.0)
            .unwrap_or_default()
    });
    let limit_x = Signal::derive(move || {
        let content_width = {
            if let Some(node) = content_ref.get() {
                node.offset_width() as f64
            } else {
                Default::default()
            }
        };
        use_document()
            .body()
            .map(|body| (body.get_bounding_client_rect().width() - content_width) - 4.0)
            .unwrap_or_default()
    });
    let open = RwSignal::new(false);
    view! {
        <ContextMenuProvider content_ref=content_ref open=open>
            <ContextMenuTrigger>
                {children()}
            </ContextMenuTrigger>
            <ContextMenuContent class="z-40 select-none" limit_y=limit_y limit_x=limit_x>
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <div
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                    >
                        "Add Reaction"
                    </div>
                    <div
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                    >
                        "Reply"
                    </div>
                    {
                        move || {
                            (current_member.get().id == member_id.get()).then(|| {
                                view!{
                                    <div
                                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                    >
                                        "Edit Message"
                                    </div>
                                }
                            })
                        }
                    }

                    {
                        current_server.member_can_edit.then(|| {
                            view!{<Pin message=message server_id=current_server.server.id(){..} on:click=move |_| open.set(false)/>}
                        })
                    }

                    <div
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                    >
                        "Create Thread"
                    </div>
                    {
                        move || {
                            (current_member.get().id == member_id.get() || current_server.member_can_edit).then(|| {
                                view!{
                                    <div
                                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                    >
                                        "Delete Message"
                                    </div>
                                }
                            })
                        }
                    }
                </div>
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}
