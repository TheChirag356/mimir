use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
