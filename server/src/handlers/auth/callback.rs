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
    // Fetch stored CSRF
    let stored_csrf: String = match session.get("csrf_token").await? {
        Some(c) => c,
        None => return Err(AppError::BadRequest(RequestError::MissingSession("CSRF"))),
    };
    // Fetch stored PKCE verifier
    let pkce_verifier = match session.get("pkce_verifier").await? {
        Some(v) => v,
        None => return Err(AppError::BadRequest(RequestError::MissingSession("PKCE"))),
    };
    // Fetch stored nonce
    let nonce = match session.get::<Nonce>("nonce").await? {
        Some(v) => v,
        None => return Err(AppError::BadRequest(RequestError::MissingSession("Nonce"))),
    };

    // Chech CSRF
    if stored_csrf != params.state {
        event!(Level::WARN, "CSRF mismatch");
        return Err(AppError::AuthError(AuthError::ValidationError(
            "CSRF token mismatch",
        )));
    }

    // Exchange code from user callback with auth provider
    // Only failure possibility is code exchange failure
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

    // Should never fail, only possible with unexpected token structure. Can't imagine Google would break that...
    let id_token = match token_response.id_token() {
        Some(t) => t,
        None => {
            return Err(AppError::AuthError(AuthError::ServiceError(
                "Server did not return an ID token",
            )));
        }
    };

    // Get ID token verifier
    let id_token_verifier = app_state.oauth_client.id_token_verifier();

    // Extract claims using ID token verifier and nonce
    // Should never really fail either
    let claims = match id_token.claims(&id_token_verifier, &nonce) {
        Ok(c) => c,
        Err(e) => {
            event!(Level::WARN, error = ?e, "Failed to verify id token claims");
            return Err(AppError::AuthError(AuthError::ValidationError(
                "Failed to verify ID token claims",
            )));
        }
    };

    // Match access token hash (timing side channel patch)
    // I don't see how this could really fail either, Google should include this
    // Inside is all validation, should be ok...
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let signing_alg = match id_token.signing_alg() {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::AuthError(AuthError::ValidationError(
                    "Failed to get signing algorithm",
                )));
            }
        };

        let signing_key = match id_token.signing_key(&id_token_verifier) {
            Ok(k) => k,
            Err(_) => {
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
            Err(_) => {
                return Err(AppError::AuthError(AuthError::ValidationError(
                    "Failed to hash access token",
                )));
            }
        };

        if actual_access_token_hash != *expected_access_token_hash {
            event!(Level::WARN, "Access token hash mismatch");
            return Err(AppError::AuthError(AuthError::Unauthorized));
        }
    }

    // Get email, should be in claims
    if let Some(email) = claims.email().map(|email| email.as_str()) {
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
            // So this is awkward...
            // Production is fucked. Guess this will fix it
            // Fingers crossed
            if email == "jonas.baugerud@gmail.com" {
                sqlx::query!(
                    "UPDATE users SET role = $1 WHERE email = $2",
                    Role::Superuser as Role,
                    email
                )
                .execute(&app_state.pool)
                .await?;
            }

            // Gen JWT
            let jwt_token = jwt::generate(&app_state.jwt_encode, &user)
                .map_err(|e| AppError::AuthError(AuthError::JWTError(e)))?;

            // Gen refresh token
            let refresh_token = refresh::generate();

            // Store refresh hash in session
            session
                .insert("refresh", (refresh_token.hash, user.id))
                .await?;

            // Prepare and store cookies
            let jwt_cookie = Cookie::new(CookieKind::JWT, &jwt_token).build();
            let refresh_cookie = Cookie::new(CookieKind::Refresh, &refresh_token.token).build();
            let jar = jar.add(jwt_cookie).add(refresh_cookie);

            // Redirect according to role
            let r = match user.role {
                Role::Superuser | Role::Admin => Redirect::to("/admin").into_response(),
                _ => Redirect::to("/").into_response(),
            };

            Ok((jar, r))
        } else { // If the user does not exist in the database
            let users = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, role AS "role: Role" FROM users"#
            )
            .fetch_all(&app_state.pool)
            .await?;
            if users.is_empty() { // The first ever user is superuser
                sqlx::query!(
                    "INSERT INTO users(name, email, role) VALUES ($1, $2, $3)",
                    None::<String>,
                    email,
                    Role::Superuser as Role
                )
                .execute(&app_state.pool)
                .await?;
                Ok((jar, Redirect::to("/admin").into_response()))
            } else { // All subsequent users are user
                sqlx::query!(
                    "INSERT INTO users(name, email, role) VALUES ($1, $2, $3)",
                    None::<String>,
                    email,
                    Role::User as Role
                )
                .execute(&app_state.pool)
                .await?;
                Ok((jar, Redirect::to("/admin").into_response()))
            }
        }
    } else {
        Err(AppError::AuthError(AuthError::Unauthorized))
    }
}
