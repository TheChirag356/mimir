use crate::db::models::{media_progress::MediaProgress, playback_session::PlaybackSession};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    library_item_id: &str,
    audio_file_id: Option<&str>,
    duration_seconds: f64,
) -> Result<PlaybackSession, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    // Resume from existing progress if there is any — a new session
    // shouldn't reset playback to zero if the user already has a
    // partially-listened position on this item.
    let existing_progress = get_progress(pool, user_id, library_item_id).await?;
    let start_time = existing_progress
        .map(|p| p.current_time_seconds)
        .unwrap_or(0.0);

    sqlx::query(
        "INSERT INTO playback_sessions
         (id, user_id, library_item_id, audio_file_id, current_time_seconds, duration_seconds, time_listened_seconds, started_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(library_item_id)
    .bind(audio_file_id)
    .bind(start_time)
    .bind(duration_seconds)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(PlaybackSession {
        id,
        user_id: user_id.to_string(),
        library_item_id: library_item_id.to_string(),
        audio_file_id: audio_file_id.map(String::from),
        current_time_seconds: start_time,
        duration_seconds,
        time_listened_seconds: 0.0,
        started_at: now,
        updated_at: now,
    })
}

pub async fn get_session(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Option<PlaybackSession>, sqlx::Error> {
    sqlx::query_as::<_, PlaybackSession>(
        "SELECT id, user_id, library_item_id, audio_file_id, current_time_seconds, duration_seconds, time_listened_seconds, started_at, updated_at
         FROM playback_sessions WHERE id = ?"
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

// This is the function that matters most: updates the session AND
// upserts media_progress in one call, so the "resume across devices"
// guarantee is always backed by the latest reported position.
pub async fn sync_progress(
    pool: &SqlitePool,
    session_id: &str,
    current_time_seconds: f64,
    time_listened_delta: f64,
) -> Result<Option<MediaProgress>, sqlx::Error> {
    let Some(session) = get_session(pool, session_id).await? else {
        return Ok(None);
    };

    let now = Utc::now().timestamp_millis();
    let new_time_listened = session.time_listened_seconds + time_listened_delta;

    sqlx::query(
        "UPDATE playback_sessions SET current_time_seconds = ?, time_listened_seconds = ?, updated_at = ? WHERE id = ?"
    )
    .bind(current_time_seconds)
    .bind(new_time_listened)
    .bind(now)
    .bind(session_id)
    .execute(pool)
    .await?;

    let progress_percent = if session.duration_seconds > 0.0 {
        (current_time_seconds / session.duration_seconds * 100.0).min(100.0)
    } else {
        0.0
    };
    // ABS's own convention: 96%+ counts as "finished" — accounts for
    // outro music/credits that a listener may not play all the way
    // through, without forcing an exact 100.0% match.
    let is_finished = progress_percent >= 96.0;

    let progress_id = Uuid::new_v4().to_string();

    // SQLite upsert — insert, but on conflict with the (user_id,
    // library_item_id) UNIQUE constraint, update instead. This is what
    // makes "last write wins" simple: whichever device calls this most
    // recently just overwrites the row.
    sqlx::query(
        "INSERT INTO media_progress
         (id, user_id, library_item_id, current_time_seconds, duration_seconds, progress_percent, is_finished, time_listened_seconds, last_update, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT (user_id, library_item_id) DO UPDATE SET
            current_time_seconds = excluded.current_time_seconds,
            duration_seconds = excluded.duration_seconds,
            progress_percent = excluded.progress_percent,
            is_finished = excluded.is_finished,
            time_listened_seconds = media_progress.time_listened_seconds + ?,
            last_update = excluded.last_update"
    )
    .bind(&progress_id)
    .bind(&session.user_id)
    .bind(&session.library_item_id)
    .bind(current_time_seconds)
    .bind(session.duration_seconds)
    .bind(progress_percent)
    .bind(is_finished)
    .bind(time_listened_delta) // for the fresh INSERT case
    .bind(now)
    .bind(now)
    .bind(time_listened_delta) // for the ON CONFLICT increment
    .execute(pool)
    .await?;

    get_progress(pool, &session.user_id, &session.library_item_id).await
}

pub async fn get_progress(
    pool: &SqlitePool,
    user_id: &str,
    library_item_id: &str,
) -> Result<Option<MediaProgress>, sqlx::Error> {
    sqlx::query_as::<_, MediaProgress>(
        "SELECT id, user_id, library_item_id, current_time_seconds, duration_seconds, progress_percent, is_finished, time_listened_seconds, last_update, created_at
         FROM media_progress WHERE user_id = ? AND library_item_id = ?"
    )
    .bind(user_id)
    .bind(library_item_id)
    .fetch_optional(pool)
    .await
}
