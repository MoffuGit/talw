#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::extract::State;
    use axum::routing::any;
    use axum_session_sqlx::SessionMySqlPool;
    use dotenvy::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use log::debug;
    use sqlx::mysql::MySqlPoolOptions;
    use start_axum::entities::user::AuthSession;
    use start_axum::entities::user::User;
    use start_axum::msg_broker::MsgBroker;
    use start_axum::state::AppState;
    use start_axum::subs::Subscriptions;
    use start_axum::uploadthing::UploadThing;
    use start_axum::ws::server::ws_handler;
    use start_axum::ws::server::WsChannels;

    use start_axum::app::*;
    use tracing::subscriber;
    use uuid::Uuid;

    use axum::{
        body::Body,
        extract::Path,
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };

    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};

    use sqlx::MySqlPool;

    use tower_cookies::{CookieManagerLayer, Cookies};
    use zeromq::Socket;
    use zeromq::SocketRecv;

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        path: Path<String>,
        cookies: Cookies,
        req: Request<Body>,
    ) -> impl IntoResponse {
        log!("path: {:?}", path);
        handle_server_fns_with_context(
            move || {
                provide_context(app_state.pool.clone());
                provide_context(app_state.uploadthing.clone());
                provide_context(cookies.clone());
                provide_context(auth_session.clone())
            },
            req,
        )
        .await
    }

    async fn leptos_router_handler(
        auth_session: AuthSession,
        state: State<AppState>,
        cookies: Cookies,
        req: Request<Body>,
    ) -> Response {
        let State(app_state) = state.clone();
        let handler = leptos_axum::render_route_with_context(
            app_state.routes.clone(),
            move || {
                provide_context(cookies.clone());
                provide_context(app_state.pool.clone());
                provide_context(app_state.uploadthing.clone());
                provide_context(auth_session.clone())
            },
            move || shell(app_state.leptos_options.clone()),
        );
        handler(state, req).await.into_response()
    }

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("have database url");

    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("connect to db");

    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<Uuid>::default();
    let session_store =
        SessionStore::<SessionMySqlPool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    sqlx::migrate!().run(&pool).await.expect("sql migrations");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let ws_channels = WsChannels::default();
    let uploadthing = UploadThing::default();
    let msg_broker = MsgBroker::new().await;
    let subscriptions = Subscriptions::default();

    let app_state = AppState {
        subscriptions,
        msg_broker: msg_broker.clone(),
        leptos_options,
        routes: routes.clone(),
        pool: pool.clone(),
        ws_channels,
        uploadthing,
    };

    let subscriber = msg_broker.subscriber.clone();
    tokio::spawn(async move {
        loop {
            let mut subscriber = subscriber.lock().await;
            let msg = subscriber.recv().await;
            match msg {
                Ok(msg) => {
                    debug!("we got a message in the msg_broker: {:?}", msg);
                }
                Err(_) => {
                    debug!("something go wrong when recv msg from the msg broker");
                }
            }
        }
    });

    let app = Router::new()
        .route("/ws", any(ws_handler))
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_router_handler))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(CookieManagerLayer::new())
        .layer(
            AuthSessionLayer::<User, Uuid, SessionMySqlPool, MySqlPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
