mod api;
mod components;
mod routes;

use api::auth::provide_auth_context;
use api::server::provide_server_context;
use api::theme::{provide_theme_context, Theme};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use routes::home::Home;
use routes::login::Login;
use routes::servers::server::Server;
use routes::servers::Servers;
use routes::signup::Signup;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_server_context();
    provide_theme_context();
    provide_auth_context();
    //NOTE:
    //[] checar los detalles en el modal, agregar iconos para subir archivos y para los templates
    //del servidor
    ////[] agregar el componente menu, dropdown o context[leptos_use para saber donde esta el mouse]
    //[] agregar distintos tipos de acciones segun el tipo de miembro
    //  [] trabajar en las invitacion al servidor
    //  [] agregar los setting del server como admin
    //[] agregar funcionalidades al usuario, setting y asi
    //[] investigar como guardar fotos en sql

    view! {
        <Stylesheet id="leptos" href="/pkg/TALW.css"/>

        <Title text="Welcome to Leptos"/>

        <Theme/>
        <main id="app" class="w-full h-full">
            <Router>
                <Routes>
                    <Route path="" view=|| view!{<Home/>}/>
                    <Route path="servers" view=|| view!{<Servers/>}>
                        <Route path=":id" view=|| view! {<Server/>} >
                            <Route path=":channel_id" view=|| view!{<div>"this is the channel page"</div>}/>
                            <Route path="" view=|| view!{<div>"in some point i want to put something here"</div>}/>
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
