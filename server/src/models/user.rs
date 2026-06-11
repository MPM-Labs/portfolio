#[derive(sqlx::Type)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "lowercase")]
pub enum Role {
    Superuser,
    Admin,
    Contributor,
}

#[derive(sqlx::Type)]
pub struct Email(String);

#[derive(sqlx::Type)]
pub struct User {
    role: Role,
    email: Email,
}
