use axum::{Extension, response::IntoResponse};

use crate::models::user::User;

pub async fn test_echo_role_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("{:?}", user.role)
}
