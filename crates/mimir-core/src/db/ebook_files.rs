use crate::db::models::ebook_file::EbookFile;
use sqlx::SqlitePool;

pub async fn get_ebook_for_item(
    pool: &SqlitePool,
    library_item_id: &str,
) -> Result<Option<EbookFile>, sqlx::Error> {
    sqlx::query_as::<_, EbookFile>(
        "SELECT id, library_item_id, file_path, format FROM ebook_files WHERE library_item_id = ?",
    )
    .bind(library_item_id)
    .fetch_optional(pool)
    .await
}
