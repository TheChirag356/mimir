use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct PlaybackSession {
    pub id: String,
    pub user_id: String,
    pub library_item_id: String,
    pub audio_file_id: Option<String>,
    pub current_time_seconds: f64,
    pub duration_seconds: f64,
    pub time_listened_seconds: f64,
    pub started_at: i64,
    pub updated_at: i64,
}
