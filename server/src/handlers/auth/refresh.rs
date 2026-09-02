use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use leptos_use::SameSite;
use tower_sessions::{Session, cookie::time::Duration};
use tracing::{Level, event, instrument};
use uuid::Uuid;

use crate::{
    auth::{
        error::AuthError,
        jwt::{self},
        refresh,
    },
    error::{AppError, RequestError},
    models::user::{Role, User},
    state::AppState,
};

#[instrument(skip_all)]
pub async fn refresh_handler(
    jar: CookieJar,
    session: Session,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = jar
        .get("refresh")
        .ok_or(AppError::AuthError(AuthError::RefreshError("Missing")))?;
    let hash = blake3::hash(refresh_token.value().as_bytes()).to_string();
    let stored_hash: (String, Uuid) =
        session
            .get("refresh")
            .await?
            .ok_or(AppError::BadRequest(RequestError::MissingSession(
                "Missing refresh token hash",
            )))?;
    if hash != stored_hash.0 {
        event!(Level::WARN, "Invalid refresh token");
        return Err(AppError::AuthError(AuthError::Unauthorized));
    }

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, role AS "role: Role" FROM users WHERE id = $1"#,
        stored_hash.1
    )
    .fetch_one(&app_state.pool)
    .await?;

    let new_jwt = jwt::generate(&app_state.jwt_encode, &user)
        .map_err(|e| AppError::AuthError(AuthError::JWTError(e)))?;

    let new_refresh = refresh::generate();
    session
        .insert("refresh", (new_refresh.hash, user.id))
        .await
        .unwrap();

    let jwt_cookie = Cookie::build(("jwt", new_jwt))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::minutes(15))
        .build();

    let refresh_cookie = Cookie::build(("refresh", new_refresh.token))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::days(30))
        .build();

    let jar = jar.add(jwt_cookie).add(refresh_cookie);

    let r = match user.role {
        Role::Superuser | Role::Admin => Redirect::to("/admin").into_response(),
        _ => Redirect::to("/").into_response(),
    };

    Ok((jar, r))
}
