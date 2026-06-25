use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct EbookFile {
    pub id: String,
    pub library_item_id: String,
    pub file_path: String,
    pub format: String,
}
