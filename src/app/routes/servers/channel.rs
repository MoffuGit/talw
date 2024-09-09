use crate::app::api::channel::{get_channel, use_channel};
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::members::{MemberSideBar, SideBarContext};
use crate::app::routes::servers::server::use_current_server_context;
use leptos::*;
use leptos_router::{use_params_map, Outlet, Redirect};
use uuid::Uuid;

#[component]
#[allow(non_snake_case)]
pub fn ChannelView() -> impl IntoView {
    let use_channel = use_channel();
    let params = use_params_map();
    let channel = create_resource(
        move || {
            (
                use_channel.rename_channel.version().get(),
                use_channel.delete_channel.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
                params.with(|p| Uuid::parse_str(p.get("channel_id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, _, server_id, channel_id)| get_channel(channel_id, server_id),
    );

    let server_id = use_current_server_context().server.id;

    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <div class="w-full h-full flex relative items-stretch">
            <div class="grow min-w-[400px] shrink-0 flex-col" >
                <Transition fallback=move || ()>
                    {move || match channel.get() {
                        Some(Ok(channel)) => view!{
                            <ChannelHeader channel=channel/>
                            <div class="w-full h-full flex overflow-hidden">
                                <div class="w-auto h-full grow"/>
                                <MemberSideBar server_id=server_id/>
                            </div>
                        }.into_view(),
                        Some(Err(_)) => view!{<Redirect path=params.with(|p| format!("/servers/{}",p.get("id").unwrap()))/>}.into_view(),
                        _ => view!{}.into_view()
                    }}
                </Transition>
            </div>
            <Outlet/>
        </div>
    }
}
