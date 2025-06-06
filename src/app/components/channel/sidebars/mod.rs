mod collapsible;
mod group;
mod profile;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::tool_tip::{
    ToolTipSide, TooltipContent, TooltipProvider, TooltipTrigger,
};
use crate::app::routes::servers::MemberStore;
use leptos::prelude::*;
use reactive_stores::Store;
use std::time::Duration;

use self::profile::Profile;

use crate::app::components::channel::sidebars::group::{Group, Groups};
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::RoleStoreStoreFields;
use crate::entities::role::RoleStoreFields;

#[derive(Debug, Clone)]
pub struct SideBarContext(pub RwSignal<bool>);

#[component]
pub fn MemberSideBar(members: Store<MemberStore>) -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    let server_context = use_current_server_context();
    let roles = server_context.roles;
    view! {
        <div
            class="h-full shrink-0 overflow-x-hidden bg-base-300 flex flex-col items-stretch justify-between ease-linear duration-200 transition-[width]"
            style=move || if open.get() { "width: 240px" } else { "width: 0px" }
        >
            <div class="flex flex-col overflow-y-scroll overflow-x-hidden items-stretch">
                <For
                    each=move || roles.roles()
                    key=|role| role.id().get()
                    let:role
                >
                    <Group members=members name=role.name() group=Groups::Online(Some(role.id().get())) />
                </For>
                <Group members=members name="Online" group=Groups::Online(None)/>
                <Group members=members name="Offline" group=Groups::Offline/>
            </div>
            <Profile />
        </div>
    }
}

#[component]
pub fn MemberSideBarTrigger() -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger
                class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer"
                on_click=Signal::derive(move || open.update(|open| *open = !*open))
            >
                <div/>
                <Icon icon=IconData::Users class="h-4 w-4"/>
            </TooltipTrigger>
            <TooltipContent
                arrow=true
                tooltip_of_side=10.0
                tip="Member List"
                tooltip_side=ToolTipSide::Bottom
                class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
            />
        </TooltipProvider>
    }
}
