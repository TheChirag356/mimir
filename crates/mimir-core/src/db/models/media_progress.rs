use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct MediaProgress {
    pub id: String,
    pub user_id: String,
    pub library_item_id: String,
    pub current_time_seconds: f64,
    pub duration_seconds: f64,
    pub progress_percent: f64,
    pub is_finished: bool,
    pub time_listened_seconds: f64,
    pub last_update: i64,
    pub created_at: i64,
}
