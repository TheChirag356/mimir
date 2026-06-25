use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct BookMetadata {
    pub id: String,
    pub library_item_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub published_year: Option<i32>,
    pub language: Option<String>,
    pub isbn: Option<String>,
    pub asin: Option<String>,
    pub explicit: bool,
}
