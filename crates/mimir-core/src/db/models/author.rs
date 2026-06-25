use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub asin: Option<String>,
    pub created_at: i64,
}
