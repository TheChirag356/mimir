use crate::db::models::library::Library;
use sqlx::SqlitePool;

pub async fn list_libraries(pool: &SqlitePool) -> Result<Vec<Library>, sqlx::Error> {
    sqlx::query_as::<_, Library>(
        "SELECT id, name, media_type, created_at, updated_at FROM libraries ORDER BY created_at ASC"
    ).fetch_all(pool)
    .await
}
