use sqlx::SqlitePool;
use uuid::Uuid;

use crate::db::models::library_folder::LibraryFolder;

pub async fn create_folder(
    pool: &SqlitePool,
    library_id: &str,
    path: &str,
) -> Result<LibraryFolder, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis();
    sqlx::query(
        "INSERT INTO library_folders (id, library_id, path, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(library_id)
    .bind(path)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(LibraryFolder {
        id,
        library_id: library_id.to_string(),
        path: path.to_string(),
        created_at: now,
    })
}

pub async fn list_folders(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<LibraryFolder>, sqlx::Error> {
    sqlx::query_as::<_, LibraryFolder>(
        "SELECT id, library_id, path, created_at FROM library_folders WHERE library_id = ?",
    )
    .bind(library_id)
    .fetch_all(pool)
    .await
}
