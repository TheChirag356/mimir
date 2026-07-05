use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct StartSessionRequest {
    pub audio_file_id: Option<String>,
    pub duration_seconds: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct SessionDto {
    pub id: String,
    pub library_item_id: String,
    pub current_time_seconds: f64,
    pub duration_seconds: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SyncProgressRequest {
    pub current_time_seconds: f64,
    pub time_listened_seconds: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProgressDto {
    pub library_item_id: String,
    pub current_time_seconds: f64,
    pub duration_seconds: f64,
    pub progress_percent: f64,
    pub is_finished: bool,
    pub last_update: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SyncEbookProgressRequest {
    // CFI = Canonical Fragment Identifier — the EPUB standard's way of
    // expressing "I am at this exact word in this chapter." Using a
    // string here rather than a float keeps it format-agnostic: your
    // Tauri webview-based reader can use a real CFI, while a simpler
    // client can just send a percentage string like "42.7" — the server
    // stores and returns whatever it received, no parsing needed.
    pub cfi: Option<String>,
    pub progress_percent: f64,
}
