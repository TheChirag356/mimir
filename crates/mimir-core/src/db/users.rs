use crate::db::models::user::User;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
    is_admin: bool,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, is_admin, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(username)
    .bind(password_hash)
    .bind(is_admin)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(User {
        id,
        username: username.to_string(),
        password_hash: password_hash.to_string(),
        is_admin,
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, is_admin, created_at, updated_at FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

pub async fn get_user(pool: &SqlitePool, id: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, is_admin, created_at, updated_at
         FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn count_users(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
