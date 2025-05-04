use leptos::prelude::*;

use crate::app::components::overview::server::ServerSettingsData;
use crate::entities::member::Member;

#[component]
pub fn MembersSettings() -> impl IntoView {
    let server = use_context::<ServerSettingsData>()
        .expect("should acces to the user overview context")
        .server;
    // let members = Resource::new(move || (), move |_| get_server_members(server.id));
    //let members = create_resource
    //for member in members
    //  name
    //  image
    //  roles

    view! {
        <div class="relative w-full h-full flex flex-col items-start">
            // <Transition>
            //     {move || {
            //         members
            //             .and_then(|members| {
            //                 members
            //                     .iter()
            //                     .map(|_member| {
            //                         view! { <p /> }
            //                     })
            //                     .collect_view()
            //             })
            //     }}
            // </Transition>
        </div>
    }
}

#[component]
fn MemberData(member: Member) -> impl IntoView {
    view! {
        // profile
        // name
        // image
        // banner
        // date of creation
        // kick out
        <div>{member.id.to_string()}</div>
    }
}
