mod auth;
mod components;
mod routes;
mod server;
mod theme;

use auth::provide_auth_context;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use routes::home::Home;
use routes::login::Login;
use routes::server::Server;
use routes::signup::Signup;
use server::provide_server_context;
use theme::{provide_theme_context, Theme};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_server_context();
    provide_theme_context();
    provide_auth_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/TALW.css"/>

        <Title text="Welcome to Leptos"/>

        <Theme/>
        <main id="app" class="w-full h-full">
            <Router>
                <Routes>
                    <Route path="" view=|| view!{<Home/>}/>
                    <Route path="servers" view=|| view!{<Server/>}>
                        <Route path=":id" view=|| view! {<div class="text-[160px] font-black">"servers stuff"</div>} />
                        <Route path="me" view=|| view! {<div>"user stuff"</div>}/>
                    </Route>
                    <Route path="login" view=move || view!{ <Login />}/>
                    <Route path="signup" view=move || view!{ <Signup />}/>
                </Routes>
            </Router>
        </main>
    }
}
