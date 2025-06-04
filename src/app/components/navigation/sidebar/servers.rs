use crate::app::components::navigation::context_server_menu::ContextServerMenu;
use crate::app::components::ui::icons::Icon;
use crate::app::components::ui::icons::IconData;
use crate::app::routes::servers::ServersStore;
use crate::app::routes::servers::ServersStoreStoreFields;
use crate::entities::server::Server;
use crate::entities::server::ServerStoreFields;
// use crate::ws::client::use_ws;
use std::time::Duration;

use crate::app::components::modal::create_server::CreateServerModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use reactive_stores::Field;
use reactive_stores::Store;

#[component]
pub fn Servers() -> impl IntoView {
    let servers_store =
        use_context::<Store<ServersStore>>().expect("should acces to the Servers Store.");

    let open = RwSignal::new(false);
    let hidden = RwSignal::new(false);
    let create_server_modal_ref = NodeRef::new();

    view! {
        <ContextMenuProvider open=open hidden=hidden>
            <ContextMenuTrigger>
                <TooltipProvider delay_duration=Duration::new(0, 0)>
                    <TooltipTrigger class="relative">
                        <A href="" {..} class=" flex relative items-center">
                            <div class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer">
                                <Icon icon=IconData::Command class="h-4 w-4 stroke-base-content" />
                            </div>
                        </A>
                    </TooltipTrigger>
                    <TooltipContent
                        tip="Servers"
                        tooltip_of_side=10.0
                        arrow=true
                        class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                    />
                </TooltipProvider>
            </ContextMenuTrigger>
            <ContextMenuContent ignore=vec![create_server_modal_ref] class="z-40 select-none">
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <CreateServerModal
                        on_open=Signal::derive(move || hidden.set(true))
                        content_ref=create_server_modal_ref
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                    >
                        <div>"Create Server"</div>
                        // <Icon icon=icondata::RiAddSystemFill />
                    </CreateServerModal>
                    <A
                        href="discover"
                        {..}
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on:click=move |_| open.set(false)
                    >
                        <div>"Discover Servers"</div>
                        // <Icon icon=icondata::RiCompass3MapFill />
                    </A>
                    <A
                        href=""
                        {..}
                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on:click=move |_| open.set(false)
                    >
                        <div>"Show Servers"</div>
                        // <Icon icon=icondata::RiCheckboxCircleSystemFill />
                    </A>
                </div>
            </ContextMenuContent>
        </ContextMenuProvider>
        <For
            each=move || servers_store.servers()
            key=|server| server.id().get()
            let:server
        >
            <ServerNavigation server=server />
        </For>
    }
}

#[component]
pub fn ServerNavigation(#[prop(into)] server: Field<Server>) -> impl IntoView {
    // use_ws().on_server_msg(server.id().get(), move |msg| {
    //     if let crate::messages::Message::ServerUpdated { name, image } = msg {
    //         if let Some(name) = name {
    //             *server.name().write() = name;
    //         }
    //         if let Some(image) = image {
    //             *server.image_url().write() = Some(image);
    //         }
    //     }
    // });
    let current_server = move || use_params_map().with(|params| params.get("id"));
    let image_url = server.image_url();
    let name = server.name();
    let id = server.id();
    view! {
        <div class="group flex relative items-center justify-center w-full">
            <div class=move || {
                format!(
                    "absolute left-0 bg-white rounded-r-full transition-all duration-100 ease-linear w-0.5 {}",
                    {
                        match current_server()
                            .is_some_and(|current| { current == id.get().simple().to_string() })
                        {
                            false => "group-hover:h-3 h-1",
                            true => "h-6",
                        }
                    },
                )
            } />
            <TooltipProvider delay_duration=Duration::new(0, 0)>
                <TooltipTrigger class="relative my-0.5">
                    <A
                        href=move || id.get().simple().to_string()
                        {..}
                        class="group flex relative items-center"
                    >
                        <ContextServerMenu server=server>
                            {move || image_url.get()
                                .map(|url| {
                                    view! { <img class="w-full h-full object-cover " src=url /> }
                                })}
                        </ContextServerMenu>
                    </A>
                </TooltipTrigger>
                <TooltipContent
                    tip=name
                    tooltip_of_side=10.0
                    arrow=true
                    class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                />
            </TooltipProvider>
        </div>
    }
}
