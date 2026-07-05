use crate::{auth::extractor::AuthUser, db, error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use mimir_common::session::{
    ProgressDto, SessionDto, StartSessionRequest, SyncEbookProgressRequest, SyncProgressRequest,
};
use uuid::Uuid;

impl From<db::models::playback_session::PlaybackSession> for SessionDto {
    fn from(s: db::models::playback_session::PlaybackSession) -> Self {
        SessionDto {
            id: s.id,
            library_item_id: s.library_item_id,
            current_time_seconds: s.current_time_seconds,
            duration_seconds: s.duration_seconds,
        }
    }
}

impl From<db::models::media_progress::MediaProgress> for ProgressDto {
    fn from(p: db::models::media_progress::MediaProgress) -> Self {
        ProgressDto {
            library_item_id: p.library_item_id,
            current_time_seconds: p.current_time_seconds,
            duration_seconds: p.duration_seconds,
            progress_percent: p.progress_percent,
            is_finished: p.is_finished,
            last_update: p.last_update,
        }
    }
}

pub async fn start_session(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    user: AuthUser,
    Json(payload): Json<StartSessionRequest>,
) -> Result<Json<SessionDto>, AppError> {
    let session = db::session::create_session(
        &state.db,
        &user.id,
        &item_id,
        payload.audio_file_id.as_deref(),
        payload.duration_seconds,
    )
    .await?;
    Ok(Json(session.into()))
}

pub async fn sync_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    _user: AuthUser,
    Json(payload): Json<SyncProgressRequest>,
) -> Result<Json<ProgressDto>, AppError> {
    let progress = db::session::sync_progress(
        &state.db,
        &session_id,
        payload.current_time_seconds,
        payload.time_listened_seconds,
    )
    .await?
    .ok_or_else(|| AppError::NotFound("session not found".into()))?;

    Ok(Json(progress.into()))
}

pub async fn get_item_progress(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    user: AuthUser,
) -> Result<Json<Option<ProgressDto>>, AppError> {
    let progress = db::session::get_progress(&state.db, &user.id, &item_id).await?;
    Ok(Json(progress.map(Into::into)))
}

pub async fn sync_ebook_progress(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    user: AuthUser,
    Json(payload): Json<SyncEbookProgressRequest>,
) -> Result<Json<ProgressDto>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let progress_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO media_progress
         (id, user_id, library_item_id, current_time_seconds, duration_seconds, progress_percent, is_finished, time_listened_seconds, last_update, created_at)
         VALUES (?, ?, ?, ?, 0, ?, ?, 0, ?, ?)
         ON CONFLICT (user_id, library_item_id) DO UPDATE SET
            current_time_seconds = excluded.current_time_seconds,
            progress_percent = excluded.progress_percent,
            is_finished = excluded.is_finished,
            last_update = excluded.last_update"
    )
    .bind(&progress_id)
    .bind(&user.id)
    .bind(&item_id)
    .bind(payload.progress_percent)
    .bind(payload.progress_percent)
    .bind(payload.progress_percent >= 96.0)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(AppError::Database)?;

    let progress = db::session::get_progress(&state.db, &user.id, &item_id)
        .await?
        .ok_or_else(|| AppError::NotFound("progress not found".into()))?;

    Ok(Json(progress.into()))
}
