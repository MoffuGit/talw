use crate::app::api::server::use_server;
use crate::app::components::navigation::server::channel::Channel;
use crate::app::components::ui::collapsible::*;
use crate::entities::member::Role;
use icondata;
use leptos::*;
use leptos_icons::*;
use uuid::Uuid;

use crate::{app::api::server::get_channels_with_category, entities::category::Category};

#[component]
pub fn Category(
    category: Category,
    server_id: Uuid,
    invite_code: Uuid,
    member_role: Role,
) -> impl IntoView {
    let is_open = create_rw_signal(false);
    //NOTE: agregar subs to resource
    let use_server = use_server();
    let create_channel_with_category = use_server.create_channel_with_category;
    let delete_channel = use_server.delete_channel;
    let rename_channel = use_server.rename_channel;

    let channels = create_resource(
        move || {
            (
                create_channel_with_category.version().get(),
                delete_channel.version().get(),
                rename_channel.version().get(),
            )
        },
        move |_| get_channels_with_category(server_id, category.id),
    );

    view! {
        <CollapsibleProvider open=is_open>
            <CollapsibleTrigger class="relative mt-4 mb-0.5">
                <div class="cursor-pointer box-border pr-2 pl-2 flex items-center justify-between group">
                    <div class="flex flex-auto overflow-hidden items-center">
                        <Icon icon=icondata::RiArrowDownSArrowsLine class=MaybeProp::derive(move || Some(TextProp::from(format!("h-4 w-4 text-base-content/75 group-hover:text-base-content {}", {
                            match is_open.get() {
                                true => "",
                                false =>"-rotate-90"
                            }
                        }))))/>
                        <div class="box-border ml-0.5 text-ellipsis whitespace-nowrap overflow-hidden uppercase text-[12px] leading-4 font-bold tracking-wide text-base-content/75 group-hover:text-base-content">
                            {category.name}
                        </div>

                        {
                            match member_role {
                                Role::ADMIN => todo!(),
                                Role::GUEST =>  view! {}.into_view()
                            }
                        }
                    </div>
                </div>
            </CollapsibleTrigger>
            <CollapsibleContent>
                {
                    move || {
                        channels.and_then(|channels| {
                            channels.iter().map(|channel| {
                                view! {<Channel channel=channel.clone() invite_code=invite_code server_id=server_id member_role=member_role/>}
                            }).collect_view()
                        })
                    }
                }
            </CollapsibleContent>
        </CollapsibleProvider>
    }
}
// view! {
//     <div>
//         <div>{category.name}</div>
//             {
//                 move || {
//                     channels.and_then(|channels| {
//                         channels.iter().map(|channel| {
//                             view! {<Channel channel=channel.clone()/>}
//                         }).collect_view()
//                     })
//                 }
//             }
//     </div>
// }
