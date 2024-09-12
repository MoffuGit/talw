mod api;
mod components;
mod routes;

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
use crate::app::routes::servers::thread_sidebar::ThreadSidebar;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_server_context();
    provide_theme_context();
    provide_auth_context();
    provide_channel_context();
    provide_category_context();

    //NOTE:
    //add the thread entity, is like
    //a channel, same values, not uses for now
    //update the sidebar with the channels and threads,
    //add capacity to create a thread
    //add the entity for Messages, add a table for pinn messages inside a channel
    //add the capacity to send messages
    //avanzar en la overview server setting and server porifle setting
    //add things to user stuff and search stuff
    //add friends

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

    //TODO:
    //the topbar is missing the threads, the pinned messages and search
    //the thread is part of the title and the group of threads
    //the messages is for the gruop of pinned messages
    //search is for latter, i want to add all the part before adding the capacity to search

    view! {
        <Stylesheet id="leptos" href="/pkg/TALW.css"/>

        <Title text="Welcome to Leptos"/>

        <Theme/>
        <main id="app" class="w-full h-full overflow-hidden">
            <Router>
                <Routes>
                    <Route path="" view=|| view!{<Home/>}/>
                    <Route path="servers" view=|| view!{<Servers/>}>
                        <Route path=":id" view=|| view! {<Server/>} >
                            <Route path=":channel_id" view=|| view!{<ChannelView/>}>
                                <Route path=":thread_id" view=|| view!{<ThreadSidebar/>}/>
                                <Route path="" view=|| view!{<div/>}/>
                            </Route>
                            // <Route path="thread/:thread_id" view=|| view!{<div/>}/>
                            <Route path="" view=|| view!{<EmptyServer/>}/>
                        </Route>
                        <Route path="me" view=|| view! {<div>"user stuff"</div>}/>
                        <Route path="search_servers" view=|| view! {<div>"search servers"</div>}/>
                    </Route>
                    <Route path="login" view=move || view!{ <Login />}/>
                    <Route path="signup" view=move || view!{ <Signup />}/>
                </Routes>
            </Router>
        </main>
    }
}
