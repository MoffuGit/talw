use std::str::FromStr;

use crate::app::api::channel::{get_channel, use_channel};
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::routes::servers::server::use_current_server_context;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::{use_params_map, Outlet, Redirect};
use uuid::Uuid;

#[component]
#[allow(non_snake_case)]
pub fn ChannelView() -> impl IntoView {
    let server_id = use_current_server_context().server.id;

    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="grow min-w-[400px] shrink-0 flex flex-col">
                {move || {
                    match use_params_map()
                        .with(|params| Uuid::from_str(params.get("channel_id").unwrap()))
                    {
                        Err(_) => {
                            view! { <Redirect path=format!("/servers/{}", server_id) /> }
                                .into_view()
                        }
                        Ok(channel_id) => {
                            let use_channel = use_channel();
                            let params = use_params_map();
                            let channel = create_resource(
                                move || {
                                    (
                                        use_channel.rename_channel.version().get(),
                                        use_channel.delete_channel.version().get(),
                                    )
                                },
                                move |(_, _)| get_channel(channel_id, server_id),
                            );

                            view! {
                                <Transition fallback=move || ()>
                                    {move || match channel.get() {
                                        Some(Ok(channel)) => {
                                            let name = channel.name.clone();
                                            view! {
                                                <ChannelHeader channel=channel />
                                                <div class="w-full h-full flex bg-base-200">
                                                    // NOTE:
                                                    // this is the future chat
                                                    // NOTE: move this to his own component,
                                                    <div class="flex flex-col h-auto w-full">
                                                        <div class="flex-grow overflow-auto" />
                                                        <div class="h-20 flex-shrink-0 flex">
                                                            <div class="m-4 w-full flex-grow bg-base-300/60 rounded-lg flex items-center px-4">
                                                                <Icon
                                                                    icon=icondata::RiAddCircleSystemFill
                                                                    class="w-7 h-7 fill-base-content/40 grow-0 mr-4"
                                                                />
                                                                <div class="grow text-base-content/60">
                                                                    {format!("Message #{}", name)}
                                                                </div>
                                                                <Icon
                                                                    icon=icondata::RiEmojiStickerCommunicationFill
                                                                    class="w-7 h-7 fill-base-content/40"
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <MemberSideBar server_id=server_id />
                                                </div>
                                            }
                                                .into_view()
                                        }
                                        Some(Err(_)) => {

                                            view! {
                                                <Redirect path=params
                                                    .with(|p| format!("/servers/{}", p.get("id").unwrap())) />
                                            }
                                                .into_view()
                                        }
                                        _ => view! {}.into_view(),
                                    }}
                                </Transition>
                            }
                        }
                    }
                }}
            </div>
            <Outlet />
        </div>
    }
}
