use rand::{RngExt, distr::Alphanumeric};

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
