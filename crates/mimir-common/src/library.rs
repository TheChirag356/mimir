use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryDto {
    pub id: String,
    pub name: String,
    pub media_type: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateLibraryRequest {
    pub name: String,
    pub media_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateLibraryRequest {
    pub name: Option<String>,
    pub media_type: Option<String>,
}
