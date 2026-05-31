use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};
use tracing::{Level, event, instrument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub iss: u64,
    pub exp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Superuser,
}

#[instrument(skip_all)]
pub fn generate(
    key: &EncodingKey,
    role: Role,
    sub: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = get_current_timestamp();

    let claims = Claims {
        sub,
        role,
        iss: now,
        exp: now - 50,
    };

    event!(Level::DEBUG, claims = ?claims, "Generating jwt");

    encode(&Header::default(), &claims, key)
}
