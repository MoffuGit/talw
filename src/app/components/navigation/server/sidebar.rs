use super::category::Category;
use super::channel::Channel;
use super::server_menu::ServerMenu;
use crate::{
    app::api::server::{get_categories, get_general_channels, get_member},
    entities::server::Server,
};
use leptos::*;

#[component]
pub fn ServerSideBar(server: Server) -> impl IntoView {
    //NOTE: las acciones las vamos a crear en el contexto del server y ya luego subscribimos los
    //resources a esas acciones aqui, create_channel, create_category,rename_member, server_settings...
    let channels = create_resource(|| (), move |_| get_general_channels(server.id));
    let categories = create_resource(|| (), move |_| get_categories(server.id));
    let member = create_resource(|| (), move |_| get_member(server.id));
    let server_name = store_value(server.name);
    view! {
        <div class="w-full h-full flex flex-col items-center relative bg-base-200 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                <Transition fallback=move || ()>
                    {
                        move || {
                            member.and_then(|member| {
                                view!{ <ServerMenu server_name=server_name.get_value() member=member.clone()/> }.into_view()
                            })
                        }
                    }
                </Transition>
                <Transition fallback=move || ()>
                    <div class="overflow-x-hidden overflow-y-scroll pr-2 flex-auto">
                        <div class="h-3"/>
                        {
                            move || {
                                channels.and_then(|channels| {
                                    channels.iter().map(|channel| {
                                        view! {<Channel channel=channel.clone()/>}
                                    }).collect_view()
                                })
                            }
                        }
                        {
                            move || {
                                categories.and_then(|categories| {
                                    categories.iter().map(|category| {
                                        view! {<Category category=category.clone() server_id=server.id/>}
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
