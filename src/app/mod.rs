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
use crate::app::routes::servers::thread::split::ThreadSplit;

use self::api::auth::use_auth;
use self::routes::servers::thread::Thread;

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
    //work in the server overview
    //add roles
    //work in the messages already pls,
    //fix the ui of the modals and user overview
    //add flallbacks for the transitions
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

    view! {
        <Stylesheet id="leptos" href="/pkg/TALW.css" />

        <Title text="Welcome to Leptos" />

        <Theme />
        <Router>
            <main id="app" class="w-full h-full overflow-hidden">
                <Routes fallback=|| "Not Found">
                    <Route path=StaticSegment("") view=Home />
                    <ProtectedParentRoute
                        condition=move || use_auth().auth.get().map(|r| r.ok().flatten().is_some())
                        redirect_path=|| "/"
                        path=StaticSegment("servers")
                        view=Servers
                    >
                        <Route
                            path=StaticSegment("")
                            view=move || view! { <div>"list of servers"</div> }
                        />
                        <Route
                            path=StaticSegment("me")
                            view=move || view! { <div>"user stuff"</div> }
                        />
                        <Route
                            path=StaticSegment("discover")
                            view=move || view! { <div>"search servers"</div> }
                        />
                        <ParentRoute path=ParamSegment("id") view=Server>
                            <Route path=StaticSegment("") view=EmptyServer />
                            <ParentRoute path=ParamSegment("channel_id") view=ChannelView>
                                <Route path=ParamSegment("thread_id") view=ThreadSplit />
                                <Route path=StaticSegment("") view=|| view! { <div /> } />
                            </ParentRoute>
                            <Route
                                path=(
                                    StaticSegment("thread"),
                                    ParamSegment("channel_id"),
                                    ParamSegment("thread_id"),
                                )
                                view=Thread
                            />
                        </ParentRoute>
                    </ProtectedParentRoute>
                    <Route path=StaticSegment("login") view=Login />
                    <Route path=StaticSegment("signup") view=Signup />
                </Routes>
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Geist:wght@100..900&display=swap"
                    rel="stylesheet"
                />
            </head>
            <body class="w-full h-screen">
                <App />
            </body>
        </html>
    }
}
