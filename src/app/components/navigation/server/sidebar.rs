use super::category::Category;
use super::channel::Channel;
use super::server_menu::ServerMenu;
use crate::{
    app::api::server::{get_categories, get_general_channels, rename_category, use_server},
    entities::{member::Member, server::Server},
};
use leptos::*;

#[component]
pub fn ServerSideBar(server: Server, member: Member) -> impl IntoView {
    //NOTE: las acciones las vamos a crear en el contexto del server y ya luego subscribimos los
    //resources a esas acciones aqui, create_channel, create_category,rename_member, server_settings...
    let use_server = use_server();
    let create_channel = use_server.create_channel;
    let delete_channel = use_server.delete_channel;
    let rename_channel = use_server.rename_channel;

    let channels = create_resource(
        move || {
            (
                create_channel.version().get(),
                delete_channel.version().get(),
                rename_channel.version().get(),
            )
        },
        move |_| get_general_channels(server.id),
    );

    let create_category = use_server.create_category;
    let delete_category = use_server.delete_category;
    let rename_category = use_server.rename_category;
    let categories = create_resource(
        move || {
            (
                create_category.version().get(),
                delete_category.version().get(),
                rename_category.version().get(),
            )
        },
        move |_| get_categories(server.id),
    );
    view! {
        <div class="w-full h-full flex flex-col items-center relative bg-base-200 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                <ServerMenu server=server.clone() member=member.clone()/>
                <Transition fallback=move || ()>
                    <div class="overflow-x-hidden overflow-y-scroll pr-2 flex-auto">
                        <div class="h-3"/>
                        {
                            move || {
                                channels.and_then(|channels| {
                                    channels.iter().map(|channel| {
                                        view! {<Channel channel=channel.clone() invite_code=server.invite_code server_id=server.id member_role=member.role/>}
                                    }).collect_view()
                                })
                            }
                        }
                        {
                            move || {
                                categories.and_then(|categories| {
                                    categories.iter().map(|category| {
                                        view! {<Category category=category.clone() server_id=server.id invite_code=server.invite_code member_role=member.role/>}
                                    }).collect_view()
                                })
                            }
                        }
                    </div>
                </Transition>
            </div>
        </div>
    }
}
