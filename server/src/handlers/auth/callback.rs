use crate::models::user::{Role, User};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, Nonce, OAuth2TokenResponse, TokenResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use tracing::{Level, event, instrument};

use crate::{
    auth::{
        cookie::{Cookie, CookieKind},
        error::AuthError,
        jwt::{self},
        refresh,
    },
    error::{AppError, RequestError},
    state::AppState,
};

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
    event!(Level::INFO, "callback: entered handler");

    // Fetch stored CSRF
    let stored_csrf: String = match session.get("csrf_token").await? {
        Some(c) => c,
        None => {
            event!(Level::ERROR, "AUTH_FAIL: missing_session_csrf");
            return Err(AppError::BadRequest(RequestError::MissingSession("CSRF")));
        }
    };
    // Fetch stored PKCE verifier
    let pkce_verifier = match session.get("pkce_verifier").await? {
        Some(v) => v,
        None => {
            event!(Level::ERROR, "AUTH_FAIL: missing_session_pkce");
            return Err(AppError::BadRequest(RequestError::MissingSession("PKCE")));
        }
    };
    // Fetch stored nonce
    let nonce = match session.get::<Nonce>("nonce").await? {
        Some(v) => v,
        None => {
            event!(Level::ERROR, "AUTH_FAIL: missing_session_nonce");
            return Err(AppError::BadRequest(RequestError::MissingSession("Nonce")));
        }
    };

    event!(Level::INFO, "callback: session values present (csrf, pkce, nonce all found)");

    // Check CSRF
    if stored_csrf != params.state {
        event!(
            Level::ERROR,
            stored_csrf = %stored_csrf,
            received_state = %params.state,
            "AUTH_FAIL: csrf_mismatch"
        );
        return Err(AppError::AuthError(AuthError::ValidationError(
            "CSRF token mismatch",
        )));
    }

    event!(Level::INFO, "callback: csrf ok, exchanging code");

    // Exchange code from user callback with auth provider
    let token_response = match app_state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|e| {
            event!(Level::ERROR, error = ?e, "AUTH_FAIL: exchange_code_configure_failed");
            AuthError::ValidationError("Failed to configure OIDC exchange")
        })?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            event!(Level::ERROR, error = ?e, "AUTH_FAIL: code_exchange_http_error");
            return Err(AppError::AuthError(AuthError::ServiceError(
                "HTTP error on code exchange",
            )));
        }
    };

    event!(Level::INFO, "callback: code exchange succeeded, extracting id_token");

    let id_token = match token_response.id_token() {
        Some(t) => t,
        None => {
            event!(Level::ERROR, "AUTH_FAIL: no_id_token_in_response");
            return Err(AppError::AuthError(AuthError::ServiceError(
                "Server did not return an ID token",
            )));
        }
    };

    // Get ID token verifier
    let id_token_verifier = app_state.oauth_client.id_token_verifier();

    event!(Level::INFO, "callback: verifying claims");

    // Extract claims using ID token verifier and nonce
    let claims = match id_token.claims(&id_token_verifier, &nonce) {
        Ok(c) => c,
        Err(e) => {
            event!(Level::ERROR, error = ?e, "AUTH_FAIL: claims_verification_failed");
            return Err(AppError::AuthError(AuthError::ValidationError(
                "Failed to verify ID token claims",
            )));
        }
    };

    event!(Level::INFO, "callback: claims verified ok");

    // Match access token hash (timing side channel patch)
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        event!(Level::INFO, "callback: access_token_hash present in claims, verifying");

        let signing_alg = match id_token.signing_alg() {
            Ok(a) => a,
            Err(e) => {
                event!(Level::ERROR, error = ?e, "AUTH_FAIL: get_signing_alg_failed");
                return Err(AppError::AuthError(AuthError::ValidationError(
                    "Failed to get signing algorithm",
                )));
            }
        };

        let signing_key = match id_token.signing_key(&id_token_verifier) {
            Ok(k) => k,
            Err(e) => {
                event!(Level::ERROR, error = ?e, "AUTH_FAIL: get_signing_key_failed");
                return Err(AppError::AuthError(AuthError::ValidationError(
                    "Failed to get signing key",
                )));
            }
        };

        let actual_access_token_hash = match AccessTokenHash::from_token(
            token_response.access_token(),
            signing_alg,
            signing_key,
        ) {
            Ok(h) => h,
            Err(e) => {
                event!(Level::ERROR, error = ?e, "AUTH_FAIL: hash_access_token_failed");
                return Err(AppError::AuthError(AuthError::ValidationError(
                    "Failed to hash access token",
                )));
            }
        };

        if actual_access_token_hash != *expected_access_token_hash {
            event!(Level::ERROR, "AUTH_FAIL: access_token_hash_mismatch");
            return Err(AppError::AuthError(AuthError::Unauthorized));
        }

        event!(Level::INFO, "callback: access_token_hash verified ok");
    } else {
        event!(Level::INFO, "callback: no access_token_hash in claims, skipping check");
    }

    // Get email, should be in claims
    if let Some(email) = claims.email().map(|email| email.as_str()) {
        event!(Level::INFO, email = %email, "callback: email found in claims, looking up user");

        // Fetch the user by email
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, name, email, role AS "role: Role" FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&app_state.pool)
        .await?;

        // If the user exists
        if let Some(user) = user {
            event!(
                Level::INFO,
                email = %email,
                user_id = ?user.id,
                role = ?user.role,
                "callback: existing user found"
            );

            // TEMP: hardcoded superuser bump for jonas — remove once confirmed fixed
            if email == "jonas.baugerud@gmail.com" {
                event!(Level::WARN, email = %email, "callback: applying hardcoded superuser bump");
                sqlx::query!(
                    "UPDATE users SET role = $1 WHERE email = $2",
                    Role::Superuser as Role,
                    email
                )
                .execute(&app_state.pool)
                .await?;
            }

            // Gen JWT
            let jwt_token = match jwt::generate(&app_state.jwt_encode, &user) {
                Ok(t) => t,
                Err(e) => {
                    event!(Level::ERROR, error = ?e, "AUTH_FAIL: jwt_generation_failed");
                    return Err(AppError::AuthError(AuthError::JWTError(e)));
                }
            };

            event!(Level::INFO, "callback: jwt generated ok");

            // Gen refresh token
            let refresh_token = refresh::generate();

            // Store refresh hash in session
            if let Err(e) = session.insert("refresh", (refresh_token.hash, user.id)).await {
                event!(Level::ERROR, error = ?e, "AUTH_FAIL: session_insert_refresh_failed");
                return Err(e.into());
            }

            event!(Level::INFO, "callback: refresh stored in session ok");

            // Prepare and store cookies
            let jwt_cookie = Cookie::new(CookieKind::JWT, &jwt_token).build();
            let refresh_cookie = Cookie::new(CookieKind::Refresh, &refresh_token.token).build();
            let jar = jar.add(jwt_cookie).add(refresh_cookie);

            // Redirect according to role
            let r = match user.role {
                Role::Superuser | Role::Admin => Redirect::to("/admin").into_response(),
                _ => Redirect::to("/").into_response(),
            };

            event!(Level::INFO, email = %email, "callback: SUCCESS, redirecting existing user");

            Ok((jar, r))
        } else {
            // If the user does not exist in the database
            event!(Level::INFO, email = %email, "callback: no existing user, checking if first user");

            let users = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, role AS "role: Role" FROM users"#
            )
            .fetch_all(&app_state.pool)
            .await?;

            if users.is_empty() {
                event!(Level::INFO, email = %email, "callback: table empty, inserting as Superuser");
                sqlx::query!(
                    "INSERT INTO users(name, email, role) VALUES ($1, $2, $3)",
                    None::<String>,
                    email,
                    Role::Superuser as Role
                )
                .execute(&app_state.pool)
                .await?;
                event!(Level::INFO, email = %email, "callback: SUCCESS, new superuser created");
                Ok((jar, Redirect::to("/admin").into_response()))
            } else {
                event!(Level::INFO, email = %email, "callback: table not empty, inserting as User");
                sqlx::query!(
                    "INSERT INTO users(name, email, role) VALUES ($1, $2, $3)",
                    None::<String>,
                    email,
                    Role::User as Role
                )
                .execute(&app_state.pool)
                .await?;
                event!(Level::INFO, email = %email, "callback: SUCCESS, new user created");
                Ok((jar, Redirect::to("/admin").into_response()))
            }
        }
    } else {
        event!(Level::ERROR, "AUTH_FAIL: no_email_in_claims");
        Err(AppError::AuthError(AuthError::Unauthorized))
    }
}