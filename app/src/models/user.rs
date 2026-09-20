use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "role"))]
#[cfg_attr(feature = "ssr", sqlx(rename_all = "lowercase"))]
pub enum Role {
    Superuser,
    Admin,
    Contributor,
    User,
}

impl From<()> for Role {
    fn from(_: ()) -> Self {
        Self::User
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Email(String);

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Email,
    pub role: Role,
}
