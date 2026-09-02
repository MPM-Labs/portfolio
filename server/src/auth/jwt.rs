use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};
use tracing::{Level, event, instrument};

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user: User,
    pub iat: u64,
    pub exp: u64,
}

#[instrument(skip_all)]
pub fn generate(key: &EncodingKey, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = get_current_timestamp();

    let claims = Claims {
        user: user.clone(),
        iat: now,
        exp: now + 300,
    };

    event!(Level::DEBUG, claims = ?claims, "Generating jwt");

    encode(&Header::default(), &claims, key)
}
