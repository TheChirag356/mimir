use sqlx::SqlitePool;
use crate::db::models::audio_file::AudioFile;

pub async fn get_audio_file(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<AudioFile>, sqlx::Error> {
    sqlx::query_as::<_, AudioFile>(
        "SELECT id, library_item_id, file_path, track_index, duration_seconds, codec, bitrate
         FROM audio_files WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// Used to confirm a given audio file actually belongs to the item the
// client claims it does — without this check, a valid-but-unrelated
// audio_file_id could be used to fetch a file outside its expected item.
pub async fn get_audio_file_for_item(
    pool: &SqlitePool,
    item_id: &str,
    audio_file_id: &str,
) -> Result<Option<AudioFile>, sqlx::Error> {
    sqlx::query_as::<_, AudioFile>(
        "SELECT id, library_item_id, file_path, track_index, duration_seconds, codec, bitrate
         FROM audio_files WHERE id = ? AND library_item_id = ?"
    )
    .bind(audio_file_id)
    .bind(item_id)
    .fetch_optional(pool)
    .await
}
