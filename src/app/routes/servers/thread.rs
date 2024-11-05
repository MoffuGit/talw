use crate::app::api::channel::{get_channel, use_channel};
use crate::app::api::thread::{get_thread, initial_width, toggle_thread_width, use_thread};
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::navigation::server::use_current_thread;
use crate::app::components::thread::sidebar::ThreadSideBar;
use crate::app::routes::servers::server::use_current_server_context;
use leptos::html::Div;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::{use_params_map, Redirect};
use leptos_use::core::Position;
use leptos_use::use_draggable_with_options;
use leptos_use::{use_window, UseDraggableCallbackArgs, UseDraggableOptions, UseDraggableReturn};

#[component]
#[allow(non_snake_case)]
pub fn ThreadSplit() -> impl IntoView {
    let initial_width = initial_width();
    let update_width = create_action(|width: &f64| toggle_thread_width(*width));

    let divider_ref = create_node_ref::<Div>();
    let UseDraggableReturn { x, .. } = use_draggable_with_options(
        divider_ref,
        UseDraggableOptions::default()
            .initial_value(Signal::derive(move || {
                let window = use_window();
                if window.is_some() {
                    let window_width = window
                        .as_ref()
                        .unwrap()
                        .inner_width()
                        .expect("should acces to thw window width");
                    if let Some(window_width) = window_width.as_f64() {
                        return Position {
                            x: window_width - initial_width,
                            y: 0.0,
                        };
                    }
                }
                Position { x: 1000.0, y: 0.0 }
            }))
            .on_end(
                move |UseDraggableCallbackArgs {
                          position: Position { x, .. },
                          ..
                      }| {
                    let window = use_window();
                    if window.is_some() {
                        let window_width = window
                            .as_ref()
                            .unwrap()
                            .inner_width()
                            .expect("should acces to thw window width");
                        if let Some(window_width) = window_width.as_f64() {
                            if x < 720.0 {
                                update_width.dispatch(window_width - 720.0);
                            }
                            if x > window_width - 400.0 {
                                update_width.dispatch(400.0);
                            }
                            if x > 0.0 {
                                update_width.dispatch(window_width - x);
                            }
                        }
                    }
                },
            ),
    );

    let current_width = move || {
        let window = use_window();
        if window.is_some() {
            let window_width = window
                .as_ref()
                .unwrap()
                .inner_width()
                .expect("should acces to thw window width");
            if let Some(window_width) = window_width.as_f64() {
                log::info!("x: {}", x.get());
                if x.get() < 720.0 {
                    return window_width - 720.0;
                }
                if x.get() > window_width - 400.0 {
                    return 400.0;
                }
                if x.get() > 0.0 {
                    return window_width - x.get();
                }
            }
        }
        initial_width
    };

    view! {
        <div class="w-2 bg-base-300 h-full shrink-0" node_ref=divider_ref/>
        <div class="min-w-[400px] shrink-0 flex" style=move || format!("width: {}px", current_width())>
            <ThreadSideBar/>
        </div>
    }
}

#[component]
pub fn ThreadView() -> impl IntoView {
    let server_id = use_current_server_context().server.id;
    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="grow min-w-[400px] shrink-0 flex flex-col" >
                {
                    move || {
                        match use_current_thread().get() {
                            None => view!{<Redirect path=format!("/servers/{}", server_id)/>}.into_view(),
                            Some((channel_id, thread_id)) => {
                                let use_channel = use_channel();
                                let use_thread = use_thread();
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
                                let thread = create_resource(move || (
                                    use_thread.delete_thread.version().get(),
                                ), move |_| get_thread(thread_id, channel_id));

                                view!{
                                    <Transition fallback=move || ()>
                                        {move || match (channel.get(), thread.get()) {
                                            (Some(Ok(channel)), Some(Ok(thread))) => {
                                                let name = channel.name.clone();
                                                view!{
                                                    <ChannelHeader channel=channel thread=thread/>
                                                    <div class="w-full h-full flex">
                                                        //NOTE:
                                                        //this is the future chat
                                                        //NOTE: move this to his own component,
                                                        <div class="flex flex-col h-auto w-full">
                                                            <div class="flex-grow overflow-auto"/>
                                                            <div class="h-20 flex-shrink-0 flex">
                                                                <div class="m-4 w-full flex-grow bg-base-300/60 rounded-lg flex items-center px-4">
                                                                    <Icon icon=icondata::RiAddCircleSystemFill class="w-7 h-7 fill-base-content/40 grow-0 mr-4" />
                                                                    <div class="grow text-base-content/60">
                                                                        {format!("Message #{}", name)}
                                                                    </div>
                                                                    <Icon icon=icondata::RiEmojiStickerCommunicationFill class="w-7 h-7 fill-base-content/40" />
                                                                </div>
                                                            </div>
                                                        </div>
                                                        <MemberSideBar server_id=server_id thread_id=thread_id/>
                                                    </div>
                                                }.into_view()

                                            },
                                            (Some(Err(_)), _) | (_,Some( Err(_))) => view!{<Redirect path=params.with(|p| format!("/servers/{}",p.get("id").unwrap()))/>}.into_view(),
                                            _ => view!{}.into_view()
                                        }}
                                    </Transition>
                                }
                            }
                        }
                    }
                }
            </div>
        </div>
    }
}
