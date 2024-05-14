use super::category::Category;
use super::channel::Channel;
use crate::{
    app::api::server::{get_categories, get_general_channels},
    entities::server::Server,
};
use leptos::*;

#[component]
pub fn ServerSideBar(server: Server) -> impl IntoView {
    let channels = create_resource(|| (), move |_| get_general_channels(server.id));
    let categories = create_resource(|| (), move |_| get_categories(server.id));
    view! {
        <div class="w-full h-full flex flex-col items-center relative bg-base-200 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <div class="w-full flex flex-col items-stretch justify-start flex-auto relative">
                <div class="relative w-full cursor-pointer">
                    <div class="relative font-medium py-3 px-4 shadow shadow-base-300/80">
                        <div class="h-6 flex items-center">
                            <div class="mr-2"/>
                            <div class="flex-1 flex items-center text-base font-bold overflow-hidden text-ellipsis whitespace-nowrap min-w-0">
                                {server.name}
                            </div>
                            <div class="relative"/>
                        </div>
                    </div>
                </div>
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
