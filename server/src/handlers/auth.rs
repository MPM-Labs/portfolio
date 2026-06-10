use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use leptos_use::SameSite;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, Nonce, OAuth2TokenResponse,
    TokenResponse,
};
use serde::Deserialize;
use tower_sessions::{Session, cookie::time::Duration};
use tracing::{Level, event, instrument};

use crate::{
    auth::{
        error::AuthError,
        jwt::{self, Role},
        refresh,
    },
    error::{AppError, RequestError},
    state::AppState,
};
use axum_extra::extract::cookie::Cookie;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

#[instrument(skip_all)]
pub async fn auth_callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let stored_csrf: String = session
        .get("csrf_token")
        .await?
        .ok_or(AppError::BadRequest(RequestError::MissingSession("CSRF")))?;

    if stored_csrf != params.state {
        event!(Level::WARN, "CSRF mismatch");
        return Err(AppError::AuthError(AuthError::ValidationError(
            "CSRF token mismatch",
        )));
    }

    let pkce_verifier = match session.get("pkce_verifier").await? {
        Some(v) => v,
        None => return Err(AppError::BadRequest(RequestError::MissingSession("PKCE"))),
    };
    let nonce = match session.get::<Nonce>("nonce").await? {
        Some(v) => v,
        None => return Err(AppError::BadRequest(RequestError::MissingSession("Nonce"))),
    };

    let token_response = match app_state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|_| AuthError::ValidationError("Failed to configure OIDC exchange"))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(t) => t,
        Err(_) => {
            return Err(AppError::AuthError(AuthError::ServiceError(
                "HTTP error on code exchange",
            )));
        }
    };

    let id_token =
        token_response
            .id_token()
            .ok_or(AppError::AuthError(AuthError::ServiceError(
                "Server did not return an ID token",
            )))?;

    let id_token_verifier = app_state.oauth_client.id_token_verifier();

    let claims = match id_token.claims(&id_token_verifier, &nonce) {
        Ok(c) => c,
        Err(e) => {
            event!(Level::WARN, error = ?e, "Failed to verify id token claims");
            return Err(AppError::AuthError(AuthError::ValidationError(
                "Failed to verify ID token claims",
            )));
        }
    };

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let signing_alg = id_token.signing_alg().map_err(|_| {
            AppError::AuthError(AuthError::ValidationError(
                "Failed to get signing algorithm",
            ))
        })?;

        let signing_key = id_token.signing_key(&id_token_verifier).map_err(|_| {
            AppError::AuthError(AuthError::ValidationError("Failed to get signing key"))
        })?;

        let actual_access_token_hash =
            AccessTokenHash::from_token(token_response.access_token(), signing_alg, signing_key)
                .map_err(|_| {
                    AppError::AuthError(AuthError::ValidationError("Failed to hash access token"))
                })?;

        if actual_access_token_hash != *expected_access_token_hash {
            event!(Level::WARN, "Access token hash mismatch");
            return Err(AppError::AuthError(AuthError::Unauthorized));
        }
    }

    if let Some(email) = claims.email().map(|email| email.as_str())
        && email == "jonas.baugerud@gmail.com"
    {
        let jwt_token = jwt::generate(&app_state.jwt_encode, Role::Superuser, email.to_string())
            .map_err(|e| AppError::AuthError(AuthError::JWTError(e)))?;

        let refresh_token = refresh::generate();

        session.insert("refresh", refresh_token.hash).await.unwrap();

        let jwt_cookie = Cookie::build(("jwt", jwt_token))
            .path("/") // TODO: change to /admin when auth works with db
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::minutes(15))
            .build();

        let refresh_cookie = Cookie::build(("refresh", refresh_token.token))
            .path("/auth/refresh")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::days(30))
            .build();

        let jar = jar.add(jwt_cookie).add(refresh_cookie);

        return Ok((jar, Redirect::to("/admin")));
    }

    Err(AppError::AuthError(AuthError::Unauthorized))
}
