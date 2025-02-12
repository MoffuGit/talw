mod api;
mod components;
mod routes;

use crate::app::api::thread::provide_thread_context;
use api::auth::provide_auth_context;
use api::channel::provide_channel_context;
use api::server::provide_server_context;
use api::theme::{provide_theme_context, Theme};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, ProtectedParentRoute, Route, Router, Routes};
use leptos_router::{path, ParamSegment, StaticSegment};
use routes::home::Home;
use routes::login::Login;
use routes::servers::channel::ChannelView;
use routes::servers::empty_server::EmptyServer;
use routes::servers::server::Server;
use routes::servers::Servers;
use routes::signup::Signup;

use crate::app::api::category::provide_category_context;
use crate::app::routes::servers::thread::ThreadSplit;

use self::api::auth::use_auth;
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
    //try with the version 0.7.0 and try cargo update and compare your cargo.toml with others
    //work in the overview for server settings
    //  - work in the members and role part
    //      - list of members
    //      - create roles
    //      - remove and add roles to members
    //change the ui for the modals
    //add the capacity to send messages and pin messages
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
        <Router>
            <main id="app" class="w-full h-full overflow-hidden">
                <ErrorBoundary fallback=move |_| view!{<div>fuck</div>}>
                    <Routes fallback=|| "Not Found">
                        <Route path=StaticSegment("") view=Home />
                            <ProtectedParentRoute
                                condition=move || use_auth().auth.get().map(|r| r.ok().flatten().is_some())
                                redirect_path=|| "/"
                                path=StaticSegment("servers" )
                                view=Servers
                            >
                                <Route path=StaticSegment("") view=move || view! { <div>"list of servers"</div> } />
                                <Route path=StaticSegment("me") view=move || view! { <div>"user stuff"</div> } />
                                <Route path=StaticSegment("discover") view=move || view! { <div>"search servers"</div> } />
                                <Route path=ParamSegment("id") view=move || view!{<div>the server</div>}/>
                                // <ParentRoute path=ParamSegment("id") view=Server>
                                //     <ParentRoute path=path!(":channel_id" ) view=ChannelView>
                                //         <Route path=path!(":thread_id" )view=ThreadSplit />
                                //         <Route path=path!("") view=move || view! { <div /> } />
                                //     </ParentRoute>
                                //     <Route
                                //         path=path!("thread/:channel_id/:thread_id")
                                //         view=ThreadView
                                //     />
                                //     <Route path=path!("/") view=EmptyServer />
                                // </ParentRoute>
                            </ProtectedParentRoute>
                            <Route path=StaticSegment("login" ) view=Login />
                            <Route path=StaticSegment("signup" ) view=Signup />
                    </Routes>
                </ErrorBoundary>
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Geist:wght@100..900&display=swap" rel="stylesheet"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
