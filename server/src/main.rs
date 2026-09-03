use crate::{
    handlers::{
        auth::{
            callback::auth_callback_handler, login::auth_login_handler, refresh::refresh_handler,
        },
    },
    middleware::jwt::jwt_validation,
};
use app::{App, shell};
use axum::{Router, middleware::from_fn_with_state, routing::get};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list, generate_route_list_with_exclusions};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::auth::cookie::secure_cookie_mode;

pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
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
    let state = AppState::new(leptos_options, pool).await;
    // Generate the list of routes in your Leptos App
    let (admin_routes, _): (Vec<_>, Vec<_>) = generate_route_list(App)
        .iter()
        .cloned()
        .partition(|i| i.path().starts_with("/admin"));

    let public_routes = generate_route_list_with_exclusions(
        App,
        Some(vec!["/api/get-role".to_string()]),
    )
    .into_iter()
    .filter(|route| !route.path().starts_with("/admin"))
    .collect();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookie_mode())
        .with_same_site(leptos_use::SameSite::Lax);

    let app = Router::new()
        .leptos_routes(&state, admin_routes, {
            let leptos_options = state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(from_fn_with_state(state.clone(), jwt_validation)) // Affects all above it. Should be cheap to clone with internally Arc'ed fields.
        .route("/auth/login", get(auth_login_handler))
        .route("/auth/callback", get(auth_callback_handler))
        .route("/auth/refresh", get(refresh_handler))
        .leptos_routes(&state, public_routes, {
            let leptos_options = state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(session_layer)
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
