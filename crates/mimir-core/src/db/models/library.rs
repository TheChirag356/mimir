use serde::Serialize;
use sqlx::FromRow;

// `FromRow` lets sqlx automatically map a database row into this struct,
// matching columns to fields by name — no manual field-by-field extraction.
#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Library {
    pub id: String,
    pub name: String,
    pub media_type: String,
    pub created_at: i64,
    pub updated_at: i64,
}
