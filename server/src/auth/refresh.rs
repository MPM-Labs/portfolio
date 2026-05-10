use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::{Validation, decode};
use leptos_use::SameSite;
use rand::{RngExt, distr::Alphanumeric};
use tower_sessions::{Session, cookie::time::Duration};

use crate::{
    auth::{
        error::AuthError,
        jwt::{self, Claims},
        refresh,
    },
    error::AppError,
    state::AppState,
};

pub struct RefreshToken {
    pub token: String,
    pub hash: String,
}

pub fn generate() -> RefreshToken {
    let mut rng = rand::rng();
    let token: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(64)
        .collect();
    let hash = blake3::hash(token.as_bytes()).to_string();
    RefreshToken { token, hash }
}

pub async fn refresh_handler(
    jar: CookieJar,
    session: Session,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let refresh_token = jar
        .get("refresh")
        .ok_or(AppError::AuthError(AuthError::RefreshError("Missing")))?;
    let hash = blake3::hash(refresh_token.value().as_bytes()).to_string();
    let stored_hash: String = session.get("refresh").await.unwrap().unwrap();
    if hash != stored_hash {
        return Err(AppError::AuthError(AuthError::Unauthorized));
    }

    let old_jwt = jar.get("jwt").unwrap().value();
    let mut val = Validation::default();
    val.validate_exp = false;
    let token_result = decode::<Claims>(old_jwt, &app_state.jwt_decode, &val);

    let token = match token_result {
        Ok(token) => token,
        Err(e) => return Err(AppError::AuthError(AuthError::JWTError(e))),
    };

    let claims = token.claims;

    let new_jwt = jwt::generate(&app_state.jwt_encode, claims.role, claims.sub).unwrap();

    let new_refresh = refresh::generate();
    session.insert("refresh", new_refresh.hash).await.unwrap();

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

    Ok((jar, Redirect::to("/admin").into_response()))
}
