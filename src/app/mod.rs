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
use routes::servers::channel::Channel;
use routes::servers::empty_server::EmptyServer;
use routes::servers::server::Server;
use routes::servers::Servers;
use routes::signup::Signup;

use crate::app::api::category::provide_category_context;
use crate::app::components::thread::ThreadSidebar;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_server_context();
    provide_theme_context();
    provide_auth_context();
    provide_channel_context();
    provide_category_context();

    //NOTE:avanzar en path=":channel_id"
    //agregar la entite para los mensajes, checar que es una thread y agregar los pinned messages
    //last thread tienen dos views, una va a ser /servers/:id/thread/:thread_id
    //y la otra va a estar dentro de :channel_id/thread/:thread_id
    //la primera va a tener componentes iguales a :channel_id con ligeros cambios
    //y la segunda va a ser una division de la pantalla, de un lado el canal del otro la thread
    //redirect the user when the path=channel_id is empty or message of empty server
    //avanzar en la overview server setting and server porifle setting
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
                            <Route path=":channel_id" view=|| view!{<Channel/>}>
                                <Route path=":thread_id" view=|| view!{<ThreadSidebar/>}/>
                                <Route path="" view=|| view!{<>}/>
                            </Route>
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
