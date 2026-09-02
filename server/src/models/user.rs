use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "lowercase")]
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

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(transparent)]
pub struct Email(String);

impl Into<String> for Email {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Email,
    pub role: Role,
}
