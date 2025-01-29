mod api;
mod components;
mod routes;

use crate::app::api::thread::provide_thread_context;
use api::auth::provide_auth_context;
use api::channel::provide_channel_context;
use api::server::provide_server_context;
use api::theme::{provide_theme_context, Theme};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use routes::home::Home;
use routes::login::Login;
use routes::servers::channel::ChannelView;
use routes::servers::empty_server::EmptyServer;
use routes::servers::server::Server;
use routes::servers::Servers;
use routes::signup::Signup;

use crate::app::api::category::provide_category_context;
use crate::app::routes::servers::thread::ThreadSplit;

use self::routes::servers::thread::ThreadView;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_server_context();
    provide_theme_context();
    provide_auth_context();
    provide_channel_context();
    provide_category_context();
    provide_thread_context();

    //NOTE:
    //work in the overview for server settings
    //  - work in the members and role part
    //      - list of members
    //      - create roles
    //      - remove and add roles to members
    //change the ui for the modals
    //add the capacity to send messages and pin messages
    //before other things, migrate from 0.6 to 0.7
    //add things to user stuff and search stuff servers stuff
    //add notifications
    //add inbox
    //add friends
    //add pinned channels and conversations
    //add the voice channel, stream sound and meaby video
    //add the optin to change the width of the sidebar
    //keep working in the profile settings
    //add the email and phone number to the user, give the posibility of change the password
    //add the option to edit the theme of the app, give some default options and more complex
    //settings
    //check tauri for creating a desktop app and cellphone app

    //TODO:user_side_bar is waiting the roles and aboout
    //the roles are needed for the sidebar and groups of users
    //the about is part of the banners

    //TODO:
    //add the right click for the member sidebar,
    //open menu with invite to server and add as friend, and send message

    //TODO:
    //the memberbanners is missing the part of member_can_edit,
    //add and remove roles,
    //add the banner for your self, button for opening the settings
    //add the send message
    //add as friend and invite to server

    view! {
        <Stylesheet id="leptos" href="/pkg/TALW.css" />

        <Title text="Welcome to Leptos" />

        <Theme />
        <main id="app" class="w-full h-full overflow-hidden">
            <Router>
                <Routes>
                    <Route path="" view=|| view! { <Home /> } />
                    <Route path="servers" view=|| view! { <Servers /> }>
                        <Route path="me" view=|| view! { <div>"user stuff"</div> } />
                        <Route path="discover" view=|| view! { <div>"search servers"</div> } />
                        <Route path="" view=|| view! { <div>"list of servers"</div> } />
                        <Route path=":id" view=|| view! { <Server /> }>
                            <Route path=":channel_id" view=|| view! { <ChannelView /> }>
                                <Route path=":thread_id" view=|| view! { <ThreadSplit /> } />
                                <Route path="" view=|| view! { <div /> } />
                            </Route>
                            <Route
                                path="thread/:channel_id/:thread_id"
                                view=|| view! { <ThreadView /> }
                            />
                            <Route path="" view=|| view! { <EmptyServer /> } />
                        </Route>
                    </Route>
                    <Route path="login" view=move || view! { <Login /> } />
                    <Route path="signup" view=move || view! { <Signup /> } />
                </Routes>
            </Router>
        </main>
    }
}
