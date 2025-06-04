#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::State;
    use axum::routing::any;
    use axum_session_sqlx::SessionMySqlPool;
    use dotenvy::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use sqlx::mysql::MySqlPoolOptions;
    use start_axum::entities::user::AuthSession;
    use start_axum::entities::user::User;
    use start_axum::state::AppState;
    use start_axum::sync::connections::UserConnections;
    use start_axum::sync::connections::UserConnectionsManager;
    use start_axum::sync::router::SyncRouter;
    use start_axum::sync::subs::SubscriptionManager;
    use start_axum::uploadthing::UploadThing;

    use start_axum::app::*;
    use start_axum::ws::ws_handler;
    use tokio::spawn;
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
                provide_context(app_state.sync_sender.clone());
                provide_context(app_state.connection_sender.clone());
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
                provide_context(app_state.sync_sender.clone());
                provide_context(app_state.connection_sender.clone());
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

    let uploadthing = UploadThing::default();

    let (sync_sender, sync_receiver) = async_broadcast::broadcast(1000);
    let (connection_sender, connection_receiver) = async_broadcast::broadcast(1000);

    let user_connections = UserConnections::default();

    let connections_manager = UserConnectionsManager::new(user_connections.clone());
    connections_manager
        .clone()
        .start_receiving(connection_receiver)
        .await;
    connections_manager.connection_cleanup().await;

    let subscriptions = SubscriptionManager::new();
    let sync_router = SyncRouter::new(subscriptions, user_connections.clone());

    spawn(async move {
        sync_router.start(sync_receiver).await;
    });

    let app_state = AppState {
        connection_sender,
        sync_sender,
        leptos_options,
        routes: routes.clone(),
        pool: pool.clone(),
        user_connections,
        uploadthing,
    };

    let app = Router::new()
        .route("/ws", any(ws_handler))
        .route(
            "/api/{*fn_name}",
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
pub fn main() {
    use leptos::mount::mount_to_body;
    use start_axum::app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
