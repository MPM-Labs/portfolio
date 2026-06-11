use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow};
use tower_sessions::Session;
use tracing::instrument;

use crate::{error::AppError, state::AppState};

#[instrument(skip_all)]
pub async fn auth_login_handler(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = state
        .oauth_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert("csrf_token", csrf_token.secret()).await?;
    session.insert("nonce", nonce.secret()).await?;
    session
        .insert("pkce_verifier", pkce_verifier.secret())
        .await?;

    Ok(Redirect::to(auth_url.as_str()))
}
