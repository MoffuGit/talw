use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::channel::Channel;
use icondata::Icon;
use leptos::*;
use leptos_icons::Icon;

#[component]
#[allow(non_snake_case)]
pub fn HeaderTitle(channel: Channel, thread: Option<String>) -> impl IntoView {
    let open = create_rw_signal(false);
    let CurrentServerContext {
        server,
        member_can_edit,
    } = use_current_server_context();
    let name = store_value(channel.name.clone());
    view! {
        <ContextMenuProvider modal=false open=open>
            <ContextMenuTrigger class="relative flex flex-row group items-center py-[6px] px-2 text-base">
                <Icon icon=Icon::from(channel.channel_type) class="w-6 h-6 mx-2"/>
                {name.get_value()}
            </ContextMenuTrigger>

            <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                <InvitePeopleModal invite_code=server.invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                </InvitePeopleModal>
                {
                    match member_can_edit {
                        true => view! {
                            <EditChannelModal channel=channel.clone() class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"  on_click=Signal::derive(move || open.set(false))>
                                <div class="group-hover:text-primary-content">"Edit Channel"</div>
                            </EditChannelModal>
                            <DeleteChannel channel=channel.clone() server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                                <div class="group-hover:text-primary-content">"Delete Channel"</div>
                            </DeleteChannel>
                        }.into_view(),
                        false => view! {}.into_view(),
                    }
                }
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}
