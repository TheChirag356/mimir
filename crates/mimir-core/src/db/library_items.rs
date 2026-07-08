use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct LibraryItemRow {
    pub id: String,
    pub library_id: String,
    pub path: String,
    pub media_type: String,
    pub title: String,
    pub duration_seconds: Option<f64>,
}

pub async fn list_items_for_library(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<LibraryItemRow>, sqlx::Error> {
    sqlx::query_as::<_, LibraryItemRow>(
        "SELECT li.id, li.library_id, li.path, li.media_type,
                COALESCE(bm.title, '') as title,
                (SELECT SUM(af.duration_seconds) FROM audio_files af WHERE af.library_item_id = li.id) as duration_seconds
         FROM library_items li
         LEFT JOIN book_metadata bm ON bm.library_item_id = li.id
         WHERE li.library_id = ?
         ORDER BY li.created_at DESC"
    )
    .bind(library_id)
    .fetch_all(pool)
    .await
}

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct LibraryItemDetail {
    pub id: String,
    pub library_id: String,
    pub path: String,
    pub media_type: String,
    pub title: String,
    pub duration_seconds: Option<f64>,
    pub audio_file_id: Option<String>,
}

pub async fn get_item_detail(
    pool: &SqlitePool,
    item_id: &str,
) -> Result<Option<LibraryItemDetail>, sqlx::Error> {
    sqlx::query_as::<_, LibraryItemDetail>(
        "SELECT
            li.id,
            li.library_id,
            li.path,
            li.media_type,
            COALESCE(bm.title, '') as title,
            (SELECT SUM(af.duration_seconds) FROM audio_files af
             WHERE af.library_item_id = li.id) as duration_seconds,
            (SELECT af.id FROM audio_files af
             WHERE af.library_item_id = li.id
             ORDER BY af.track_index ASC LIMIT 1) as audio_file_id
         FROM library_items li
         LEFT JOIN book_metadata bm ON bm.library_item_id = li.id
         WHERE li.id = ?",
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await
}
