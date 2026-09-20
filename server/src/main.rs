use crate::{
    handlers::auth::{
        callback::auth_callback_handler, login::auth_login_handler, refresh::refresh_handler,
    },
    middleware::jwt::jwt_validation,
};
use app::{App, shell};
use axum::{
    Extension, Router,
    extract::Request,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use leptos::{prelude::*, server_fn::axum::server_fn_paths};
use leptos_axum::{
    AxumRouteListing, generate_route_list, generate_route_list_with_exclusions,
    handle_server_fns_with_context, render_app_to_stream_with_context,
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::auth::cookie::secure_cookie_mode;

pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Should be able to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Should be able to apply migrations");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let state = AppState::new(leptos_options, pool.clone()).await;

    let server_fn_handler = {
        let state = state.clone();
        move |req: Request| {
            let state = state.clone();
            async move {
                handle_server_fns_with_context(move || provide_context(state.clone()), req).await
            }
        }
    };

    let (admin_fns, public_fns): (Vec<_>, Vec<_>) =
        server_fn_paths().partition(|(path, _method)| path.starts_with("/admin"));

    let mut admin_router: Router<AppState> = Router::new();
    for (path, _) in admin_fns {
        admin_router = admin_router.route(path, post(server_fn_handler.clone()));
    }

    let mut public_router: Router<AppState> = Router::new();
    for (path, _) in public_fns {
        public_router = public_router.route(path, post(server_fn_handler.clone()));
    }

    // Generate the list of routes in your Leptos App
    let (admin_routes, _): (Vec<_>, Vec<_>) = generate_route_list(App)
        .iter()
        .cloned()
        .partition(|i| i.path().starts_with("/admin"));

    let public_routes: Vec<AxumRouteListing> =
        generate_route_list_with_exclusions(App, Some(vec!["/api/get-role".to_string()]))
            .into_iter()
            .filter(|route| !route.path().starts_with("/admin"))
            .collect();

    let page_handler = {
        let state = state.clone();
        let leptos_options = state.leptos_options.clone();
        move || {
            let state = state.clone();
            let leptos_options = leptos_options.clone();
            render_app_to_stream_with_context(
                move || provide_context(state.clone()),
                move || shell(leptos_options.clone()),
            )
        }
    };

    let mut admin_pages: Router<AppState> = Router::new();
    for route in &admin_routes {
        admin_pages = admin_pages.route(route.path(), get(page_handler()));
    }

    let mut public_pages: Router<AppState> = Router::new();
    for route in &public_routes {
        public_pages = public_pages.route(route.path(), get(page_handler()));
    }

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookie_mode())
        .with_same_site(leptos_use::SameSite::Lax);

    let admin = admin_router
        .merge(admin_pages)
        .layer(from_fn_with_state(state.clone(), jwt_validation));

    let public = public_router.merge(public_pages);

    let app = Router::new()
        .route("/auth/login", get(auth_login_handler))
        .route("/auth/callback", get(auth_callback_handler))
        .route("/auth/refresh", get(refresh_handler))
        .merge(public)
        .merge(admin)
        .layer(Extension(pool))
        .layer(session_layer)
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
