use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct LibraryFolder {
    pub id: String,
    pub library_id: String,
    pub path: String,
    pub created_at: i64,
}
