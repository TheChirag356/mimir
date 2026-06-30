pub mod discover;
pub mod file_types;

use crate::db::models::library_folder::LibraryFolder;
use chrono::Utc;
use discover::discover_items;
use sqlx::SqlitePool;
use std::path::PathBuf;
use uuid::Uuid;

pub struct ScanSummary {
    pub items_added: usize,
    pub items_skipped: usize,
}

pub async fn scan_library(pool: &SqlitePool, library_id: &str) -> Result<ScanSummary, sqlx::Error> {
    let folders: Vec<LibraryFolder> = sqlx::query_as(
        "SELECT id, library_id, path, created_at FROM library_folders WHERE library_id = ?",
    )
    .bind(library_id)
    .fetch_all(pool)
    .await?;

    let mut summary = ScanSummary {
        items_added: 0,
        items_skipped: 0,
    };

    for folder in folders {
        let folder_path = PathBuf::from(&folder.path);

        // `spawn_blocking` hands this synchronous, file-I/O-heavy work
        // off to a dedicated thread pool, so it doesn't block Tokio's
        // async worker threads while it walks the filesystem and parses
        // audio tags. We `.await` the JoinHandle to get the result back
        // once it's done.
        let discovered = tokio::task::spawn_blocking(move || discover_items(&folder_path))
            .await
            .unwrap_or_default();

        for item in discovered {
            // Skip if we've already recorded this exact path — keeps
            // re-running a scan idempotent rather than duplicating rows.
            // Detecting *removed* or *changed* items is a follow-up
            // improvement, not handled in this first pass.
            let existing: Option<(String,)> =
                sqlx::query_as("SELECT id FROM library_items WHERE path = ?")
                    .bind(&item.path)
                    .fetch_optional(pool)
                    .await?;

            if existing.is_some() {
                summary.items_skipped += 1;
                continue;
            }

            let item_id = Uuid::new_v4().to_string();
            let now = Utc::now().timestamp_millis();
            let media_type = if item.audio_files.is_empty() {
                "book"
            } else {
                "book"
            }; // podcasts come later

            sqlx::query(
                "INSERT INTO library_items (id, library_id, folder_id, path, media_type, is_missing, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, 0, ?, ?)"
            )
            .bind(&item_id)
            .bind(library_id)
            .bind(&folder.id)
            .bind(&item.path)
            .bind(media_type)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await?;

            let metadata_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO book_metadata (id, library_item_id, title, explicit)
                 VALUES (?, ?, ?, 0)",
            )
            .bind(&metadata_id)
            .bind(&item_id)
            .bind(&item.title)
            .execute(pool)
            .await?;

            for audio in &item.audio_files {
                sqlx::query(
                    "INSERT INTO audio_files (id, library_item_id, file_path, track_index, duration_seconds, codec, bitrate)
                     VALUES (?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&item_id)
                .bind(&audio.file_path)
                .bind(audio.track_index)
                .bind(audio.duration_seconds)
                .bind(&audio.codec)
                .bind(audio.bitrate)
                .execute(pool)
                .await?;
            }

            // ebook_files has a UNIQUE constraint on library_item_id, so
            // only persist the first one found if there happen to be
            // multiple ebook files in one folder.
            if let Some(ebook) = item.ebook_files.first() {
                sqlx::query(
                    "INSERT INTO ebook_files (id, library_item_id, file_path, format)
                     VALUES (?, ?, ?, ?)",
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&item_id)
                .bind(&ebook.file_path)
                .bind(&ebook.format)
                .execute(pool)
                .await?;
            }

            summary.items_added += 1;
        }
    }

    Ok(summary)
}
