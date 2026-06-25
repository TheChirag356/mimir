use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct AudioFile {
    pub id: String,
    pub library_item_id: String,
    pub file_path: String,
    pub track_index: i32,
    pub duration_seconds: f64,
    pub codec: Option<String>,
    pub bitrate: Option<i32>,
}
