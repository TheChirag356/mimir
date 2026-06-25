use crate::db::models::library::Library;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_libraries(pool: &SqlitePool) -> Result<Vec<Library>, sqlx::Error> {
    sqlx::query_as::<_, Library>(
        "SELECT id, name, media_type, created_at, updated_at FROM libraries ORDER BY created_at ASC"
    ).fetch_all(pool)
    .await
}

pub async fn get_library(pool: &SqlitePool, id: &str) -> Result<Option<Library>, sqlx::Error> {
    sqlx::query_as::<_, Library>(
        "SELECT id, name, media_type, created_at, updated_at FROM libraries WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_library(
    pool: &SqlitePool,
    name: &str,
    media_type: &str,
) -> Result<Library, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    sqlx::query(
        "
        INSERT INTO libraries (id, name, media_type, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(media_type)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(Library {
        id,
        name: name.to_string(),
        media_type: media_type.to_string(),
        created_at: now,
        updated_at: now,
    })
}

pub async fn update_library(
    pool: &SqlitePool,
    id: &str,
    name: Option<&str>,
    media_type: Option<&str>,
) -> Result<Option<Library>, sqlx::Error> {
    let Some(existing) = get_library(pool, id).await? else {
        return Ok(None);
    };
    let new_name = name.unwrap_or(&existing.name);
    let new_media_type = media_type.unwrap_or(&existing.media_type);
    let now = Utc::now().timestamp_millis();

    sqlx::query("UPDATE libraries SET name = ?, media_type = ?,  updated_at = ? WHERE id = ?")
        .bind(new_name)
        .bind(new_media_type)
        .bind(now)
        .bind(&id)
        .execute(pool)
        .await?;

    get_library(pool, id).await
}

pub async fn delete_library(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM libraries WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
