use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{Validation, decode, errors::ErrorKind as jwtErrorKind};
use tracing::{Level, event, instrument};

use crate::{
    auth::{
        error::AuthError,
        jwt::{Claims, Role},
    },
    error::AppError,
    state::AppState,
};

#[instrument(skip_all)]
pub async fn jwt_validation(
    State(state): State<AppState>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token_str = match jar.get("jwt") {
        Some(t) => t.value(),
        None => {
            event!(Level::DEBUG, "No jwt cookie present, redirecting to /login");
            return Ok(Redirect::to("/login").into_response());
        }
    };

    event!(Level::DEBUG, "JWT cookie present");

    let token_result = decode::<Claims>(token_str, &state.jwt_decode, &Validation::default());

    let token = match token_result {
        Ok(token) => token,
        Err(e) => match e.kind() {
            jwtErrorKind::ExpiredSignature => {
                event!(Level::DEBUG, "jwt expired, redirecting to /auth/refresh");
                return Ok(Redirect::to("/auth/refresh").into_response());
            }
            _ => {
                event!(Level::ERROR, error = ?e, "JWT error");
                return Err(AppError::AuthError(AuthError::JWTError(e)));
            }
        },
    };

    event!(Level::DEBUG, role = ?token.claims.role, "JWT valid");

    match token.claims.role {
        Role::Superuser => Ok(next.run(req).await),
        #[allow(unreachable_patterns)]
        r => {
            event!(Level::WARN, role = ?r, "Unauthorized role");
            Err(AppError::AuthError(AuthError::Unauthorized))
        }
    }
}
