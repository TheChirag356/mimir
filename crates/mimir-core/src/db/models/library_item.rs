use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct LibraryItem {
    pub id: String,
    pub library_id: String,
    pub folder_id: String,
    pub path: String,
    pub media_type: String,
    pub is_missing: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
